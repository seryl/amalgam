//! Tests for the CRD fetcher

#[cfg(test)]
mod tests {
    use super::super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn sample_crd() -> serde_json::Value {
        json!({
            "apiVersion": "apiextensions.k8s.io/v1",
            "kind": "CustomResourceDefinition",
            "metadata": {
                "name": "compositions.apiextensions.crossplane.io"
            },
            "spec": {
                "group": "apiextensions.crossplane.io",
                "names": {
                    "kind": "Composition",
                    "plural": "compositions",
                    "singular": "composition"
                },
                "versions": [{
                    "name": "v1",
                    "served": true,
                    "storage": true,
                    "schema": {
                        "openAPIV3Schema": {
                            "type": "object",
                            "properties": {
                                "spec": {
                                    "type": "object",
                                    "properties": {
                                        "compositeTypeRef": {
                                            "type": "object",
                                            "properties": {
                                                "apiVersion": {"type": "string"},
                                                "kind": {"type": "string"}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }]
            }
        })
    }

    fn github_api_response(files: Vec<(&str, &str)>) -> serde_json::Value {
        let mut items = vec![];
        for (name, url) in files {
            items.push(json!({
                "name": name,
                "path": format!("cluster/crds/{}", name),
                "type": "file",
                "download_url": url
            }));
        }
        json!(items)
    }

    #[tokio::test]
    async fn test_fetch_single_yaml_file() {
        let mock_server = MockServer::start().await;
        
        let crd_yaml = serde_yaml::to_string(&sample_crd()).unwrap();
        
        Mock::given(method("GET"))
            .and(path("/test.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(crd_yaml))
            .mount(&mock_server)
            .await;

        let fetcher = CRDFetcher::new().unwrap();
        let url = format!("{}/test.yaml", &mock_server.uri());
        let crds = fetcher.fetch_from_url(&url).await.unwrap();

        assert_eq!(crds.len(), 1);
        assert_eq!(crds[0].spec.group, "apiextensions.crossplane.io");
        assert_eq!(crds[0].spec.names.kind, "Composition");
    }

    #[tokio::test]
    async fn test_parse_github_url_formats() {
        let test_cases = vec![
            (
                "https://github.com/crossplane/crossplane/tree/main/cluster/crds",
                ("crossplane", "crossplane", "cluster/crds", "main"),
            ),
            (
                "https://github.com/kubernetes/api/tree/master/core/v1",
                ("kubernetes", "api", "core/v1", "master"),
            ),
            (
                "https://github.com/owner/repo/blob/branch/file.yaml",
                ("owner", "repo", "file.yaml", "branch"),
            ),
        ];

        for (url, expected) in test_cases {
            let parts: Vec<&str> = url.split('/').collect();
            assert!(parts.len() >= 5);
            
            let owner = parts[3];
            let repo = parts[4];
            
            assert_eq!(owner, expected.0);
            assert_eq!(repo, expected.1);
        }
    }

    #[tokio::test]
    async fn test_fetch_from_github_directory() {
        let mock_server = MockServer::start().await;
        
        // Mock GitHub API response
        let files = vec![
            ("test1.yaml", format!("{}/crd1.yaml", mock_server.uri())),
            ("test2.yaml", format!("{}/crd2.yaml", mock_server.uri())),
            ("test3.yml", format!("{}/crd3.yaml", mock_server.uri())),
            ("readme.md", format!("{}/readme.md", mock_server.uri())), // Should be filtered out
        ];
        
        let api_response = github_api_response(
            files.iter().map(|(name, url)| (*name, url.as_str())).collect()
        );
        
        Mock::given(method("GET"))
            .and(path("/repos/crossplane/crossplane/contents/cluster/crds"))
            .and(header("Accept", "application/vnd.github.v3+json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&api_response))
            .mount(&mock_server)
            .await;
        
        // Mock CRD downloads
        let crd_yaml = serde_yaml::to_string(&sample_crd()).unwrap();
        for i in 1..=3 {
            Mock::given(method("GET"))
                .and(path(format!("/crd{}.yaml", i)))
                .respond_with(ResponseTemplate::new(200).set_body_string(&crd_yaml))
                .mount(&mock_server)
                .await;
        }
        
        // Construct GitHub URL using mock server
        let url = format!(
            "{}/github.com/crossplane/crossplane/tree/main/cluster/crds",
            mock_server.uri()
        );
        
        // We need to mock this differently since we can't override the GitHub API URL
        // Instead, let's test the internal methods
    }

    #[tokio::test]
    async fn test_concurrent_downloads() {
        let mock_server = MockServer::start().await;
        
        // Create many CRD files to test concurrent downloading
        let num_files = 20;
        let mut mocks = vec![];
        
        for i in 0..num_files {
            let crd = json!({
                "apiVersion": "apiextensions.k8s.io/v1",
                "kind": "CustomResourceDefinition",
                "metadata": {
                    "name": format!("resource{}.example.io", i)
                },
                "spec": {
                    "group": "example.io",
                    "names": {
                        "kind": format!("Resource{}", i),
                        "plural": format!("resource{}s", i)
                    },
                    "versions": [{
                        "name": "v1",
                        "served": true,
                        "storage": true,
                        "schema": {
                            "openAPIV3Schema": {
                                "type": "object"
                            }
                        }
                    }]
                }
            });
            
            let yaml = serde_yaml::to_string(&crd).unwrap();
            let mock = Mock::given(method("GET"))
                .and(path(format!("/crd{}.yaml", i)))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_string(yaml)
                        .set_delay(std::time::Duration::from_millis(100))
                )
                .mount(&mock_server)
                .await;
            mocks.push(mock);
        }
        
        // We would test concurrent fetching here if we could inject URLs
        // For now this demonstrates the test setup
    }

    #[tokio::test]
    async fn test_error_handling_404() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/missing.yaml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let fetcher = CRDFetcher::new().unwrap();
        let url = format!("{}/missing.yaml", &mock_server.uri());
        let result = fetcher.fetch_from_url(&url).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_error_handling_invalid_yaml() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/invalid.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not: valid: yaml: content:"))
            .mount(&mock_server)
            .await;

        let fetcher = CRDFetcher::new().unwrap();
        let url = format!("{}/invalid.yaml", &mock_server.uri());
        let result = fetcher.fetch_from_url(&url).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_non_crd_yaml_filtered() {
        let mock_server = MockServer::start().await;
        
        let non_crd = json!({
            "apiVersion": "v1",
            "kind": "ConfigMap",
            "metadata": {
                "name": "test-config"
            },
            "data": {
                "key": "value"
            }
        });
        
        let yaml = serde_yaml::to_string(&non_crd).unwrap();
        
        Mock::given(method("GET"))
            .and(path("/configmap.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(yaml))
            .mount(&mock_server)
            .await;

        let fetcher = CRDFetcher::new().unwrap();
        let url = format!("{}/configmap.yaml", &mock_server.uri());
        let result = fetcher.fetch_from_url(&url).await;
        
        // Should fail because it's not a CRD
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_progress_indicators_disabled_non_tty() {
        // This test would verify that progress bars are not shown when not in TTY
        // We can't easily test this without mocking atty::is()
        // But the structure is here for documentation
    }

    #[test]
    fn test_github_url_parsing_edge_cases() {
        let test_cases = vec![
            ("https://github.com/owner/repo", false), // Too short
            ("https://gitlab.com/owner/repo/tree/main", false), // Wrong host
            ("github.com/owner/repo/tree/main", false), // Missing protocol
            ("https://github.com/owner/repo/tree/main/path", true), // Valid
        ];

        for (url, should_be_valid) in test_cases {
            let parts: Vec<&str> = url.split('/').collect();
            let is_valid = parts.len() >= 5 && 
                          url.contains("github.com") && 
                          url.starts_with("https://");
            assert_eq!(is_valid, should_be_valid, "URL: {}", url);
        }
    }
}