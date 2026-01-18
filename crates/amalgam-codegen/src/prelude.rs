//! Prelude generation for Nickel packages
//!
//! Generates a unified entry point (`prelude.ncl`) that re-exports all types
//! and provides convenient shortcuts and helper functions.
//!
//! # Example Generated Prelude
//!
//! ```nickel,ignore
//! # Generated prelude.ncl
//! {
//!   k8s = {
//!     core.v1 = import "./k8s_io/api/core/v1.ncl",
//!     apps.v1 = import "./k8s_io/api/apps/v1.ncl",
//!     # ...
//!
//!     # Shortcuts for common types
//!     Deployment = apps.v1.Deployment,
//!     Service = core.v1.Service,
//!     Pod = core.v1.Pod,
//!   },
//!
//!   helpers = import "./lib/helpers.ncl",
//!   mixins = import "./lib/mixins.ncl",
//! }
//! ```

use std::fmt::Write;
use std::path::{Path, PathBuf};

/// Configuration for prelude generation
#[derive(Debug, Clone)]
pub struct PreludeConfig {
    /// Name of the prelude file (default: "prelude.ncl")
    pub filename: String,
    /// Generate K8s shortcuts (Deployment, Service, Pod, etc.)
    pub k8s_shortcuts: bool,
    /// Generate helper functions module
    pub generate_helpers: bool,
    /// Generate mixins module
    pub generate_mixins: bool,
    /// Custom type shortcuts: (shortcut_name, full_path)
    pub custom_shortcuts: Vec<(String, String)>,
    /// Packages to include in the prelude
    pub packages: Vec<PackageEntry>,
}

impl Default for PreludeConfig {
    fn default() -> Self {
        Self {
            filename: "prelude.ncl".to_string(),
            k8s_shortcuts: true,
            generate_helpers: true,
            generate_mixins: true,
            custom_shortcuts: Vec::new(),
            packages: Vec::new(),
        }
    }
}

/// Entry for a package to include in the prelude
#[derive(Debug, Clone)]
pub struct PackageEntry {
    /// Package name (e.g., "k8s_io", "crossplane")
    pub name: String,
    /// Path relative to the prelude (e.g., "./k8s_io")
    pub path: PathBuf,
    /// API groups discovered in this package
    pub api_groups: Vec<ApiGroup>,
}

/// API group within a package
#[derive(Debug, Clone)]
pub struct ApiGroup {
    /// Group name (e.g., "core", "apps", "batch")
    pub name: String,
    /// Versions available (e.g., ["v1", "v1beta1"])
    pub versions: Vec<String>,
    /// Path to the group directory relative to package
    pub path: PathBuf,
}

/// Generator for prelude.ncl files
pub struct PreludeGenerator {
    config: PreludeConfig,
}

impl PreludeGenerator {
    pub fn new(config: PreludeConfig) -> Self {
        Self { config }
    }

    /// Discover packages from a root directory
    pub fn discover_packages(root: &Path) -> Vec<PackageEntry> {
        let mut packages = Vec::new();

        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();

                    // Skip hidden directories and special files
                    if name.starts_with('.') || name == "lib" {
                        continue;
                    }

                    // Check if it's a valid package (has mod.ncl or Nickel-pkg.ncl)
                    if path.join("mod.ncl").exists() || path.join("Nickel-pkg.ncl").exists() {
                        let api_groups = Self::discover_api_groups(&path);
                        packages.push(PackageEntry {
                            name: name.clone(),
                            path: PathBuf::from(format!("./{}", name)),
                            api_groups,
                        });
                    }
                }
            }
        }

        packages
    }

    /// Discover API groups within a package
    fn discover_api_groups(package_path: &Path) -> Vec<ApiGroup> {
        let mut groups = Vec::new();

        // Check for api/ subdirectory (K8s-style)
        let api_path = package_path.join("api");
        if api_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&api_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let group_name = entry.file_name().to_string_lossy().to_string();
                        let versions = Self::discover_versions(&path);

                        if !versions.is_empty() {
                            groups.push(ApiGroup {
                                name: group_name.clone(),
                                versions,
                                path: PathBuf::from(format!("api/{}", group_name)),
                            });
                        }
                    }
                }
            }
        }

        // Check for version directories directly in package (CRD-style)
        if let Ok(entries) = std::fs::read_dir(package_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    // Check if it looks like a version directory
                    if name.starts_with('v')
                        && name.len() > 1
                        && name.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
                    {
                        // This is a version directory directly in the package
                        if groups.iter().all(|g| g.name != "default") {
                            groups.push(ApiGroup {
                                name: "default".to_string(),
                                versions: vec![name],
                                path: PathBuf::from("."),
                            });
                        } else if let Some(default_group) =
                            groups.iter_mut().find(|g| g.name == "default")
                        {
                            default_group.versions.push(name);
                        }
                    }
                }
            }
        }

        groups
    }

    /// Discover versions within an API group directory
    fn discover_versions(group_path: &Path) -> Vec<String> {
        let mut versions = Vec::new();

        if let Ok(entries) = std::fs::read_dir(group_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                // Check for version files (v1.ncl) or version directories (v1/)
                if path.is_file() && name.ends_with(".ncl") {
                    let version = name.trim_end_matches(".ncl").to_string();
                    if version.starts_with('v') {
                        versions.push(version);
                    }
                } else if path.is_dir()
                    && name.starts_with('v')
                    && name.len() > 1
                    && name.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
                {
                    versions.push(name);
                }
            }
        }

        // Sort versions semantically
        versions.sort_by(|a, b| {
            // Parse version components for proper sorting
            let parse_version = |s: &str| -> (i32, i32, i32) {
                let s = s.trim_start_matches('v');
                let mut parts = s.split(|c: char| !c.is_ascii_digit());
                let major = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
                let minor = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
                let patch = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
                (major, minor, patch)
            };

            let (a_major, a_minor, a_patch) = parse_version(a);
            let (b_major, b_minor, b_patch) = parse_version(b);

            // Sort stable versions before alpha/beta
            let a_is_stable = !a.contains("alpha") && !a.contains("beta");
            let b_is_stable = !b.contains("alpha") && !b.contains("beta");

            match (a_is_stable, b_is_stable) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => (a_major, a_minor, a_patch).cmp(&(b_major, b_minor, b_patch)),
            }
        });

        versions
    }

    /// Generate the prelude.ncl content
    pub fn generate(&self) -> String {
        let mut output = String::new();

        writeln!(output, "# Generated prelude.ncl").unwrap();
        writeln!(output, "# Provides unified entry point for all type packages").unwrap();
        writeln!(output, "#").unwrap();
        writeln!(output, "# Usage:").unwrap();
        writeln!(
            output,
            "#   let {{ k8s, helpers, ... }} = import \"./prelude.ncl\" in"
        )
        .unwrap();
        writeln!(output, "#   {{ deployment = k8s.Deployment & ... }}").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "{{").unwrap();

        // Generate package imports
        for package in &self.config.packages {
            self.generate_package_import(package, &mut output);
        }

        // Generate K8s shortcuts if enabled
        if self.config.k8s_shortcuts {
            self.generate_k8s_shortcuts(&mut output);
        }

        // Generate custom shortcuts
        if !self.config.custom_shortcuts.is_empty() {
            writeln!(output).unwrap();
            writeln!(output, "  # Custom shortcuts").unwrap();
            for (name, path) in &self.config.custom_shortcuts {
                writeln!(output, "  {} = {},", name, path).unwrap();
            }
        }

        // Generate helpers import
        if self.config.generate_helpers {
            writeln!(output).unwrap();
            writeln!(output, "  # Helper functions").unwrap();
            writeln!(output, "  helpers = import \"./lib/helpers.ncl\",").unwrap();
        }

        // Generate mixins import
        if self.config.generate_mixins {
            writeln!(output).unwrap();
            writeln!(output, "  # Mixins for common patterns").unwrap();
            writeln!(output, "  mixins = import \"./lib/mixins.ncl\",").unwrap();
        }

        writeln!(output, "}}").unwrap();

        output
    }

    /// Generate import statements for a package
    fn generate_package_import(&self, package: &PackageEntry, output: &mut String) {
        let sanitized_name = package.name.replace(['.', '-'], "_");

        if package.api_groups.is_empty() {
            // Simple package import
            writeln!(
                output,
                "  {} = import \"{}/mod.ncl\",",
                sanitized_name,
                package.path.display()
            )
            .unwrap();
        } else {
            // Package with API groups
            writeln!(output, "  {} = {{", sanitized_name).unwrap();

            for group in &package.api_groups {
                self.generate_api_group_import(package, group, output);
            }

            writeln!(output, "  }},").unwrap();
        }
    }

    /// Generate import statements for an API group
    fn generate_api_group_import(
        &self,
        package: &PackageEntry,
        group: &ApiGroup,
        output: &mut String,
    ) {
        let sanitized_group = group.name.replace(['.', '-'], "_");

        if group.versions.len() == 1 {
            // Single version - import directly
            let version = &group.versions[0];
            let import_path = if group.path.as_os_str() == "." {
                format!("{}/{}.ncl", package.path.display(), version)
            } else {
                format!("{}/{}/{}.ncl", package.path.display(), group.path.display(), version)
            };
            writeln!(
                output,
                "    {}.{} = import \"{}\",",
                sanitized_group, version, import_path
            )
            .unwrap();
        } else {
            // Multiple versions - create nested structure
            writeln!(output, "    {} = {{", sanitized_group).unwrap();
            for version in &group.versions {
                let import_path = if group.path.as_os_str() == "." {
                    format!("{}/{}.ncl", package.path.display(), version)
                } else {
                    format!("{}/{}/{}.ncl", package.path.display(), group.path.display(), version)
                };
                writeln!(output, "      {} = import \"{}\",", version, import_path).unwrap();
            }
            writeln!(output, "    }},").unwrap();
        }
    }

    /// Generate K8s type shortcuts
    fn generate_k8s_shortcuts(&self, output: &mut String) {
        // Find k8s_io package
        let k8s_package = self.config.packages.iter().find(|p| p.name == "k8s_io");

        if k8s_package.is_none() {
            return;
        }

        writeln!(output).unwrap();
        writeln!(output, "  # Common K8s type shortcuts").unwrap();

        // Define common shortcuts
        let shortcuts = [
            // Core types
            ("Pod", "k8s_io.core.v1.Pod"),
            ("Service", "k8s_io.core.v1.Service"),
            ("ConfigMap", "k8s_io.core.v1.ConfigMap"),
            ("Secret", "k8s_io.core.v1.Secret"),
            ("PersistentVolumeClaim", "k8s_io.core.v1.PersistentVolumeClaim"),
            ("Namespace", "k8s_io.core.v1.Namespace"),
            ("ServiceAccount", "k8s_io.core.v1.ServiceAccount"),
            // Apps types
            ("Deployment", "k8s_io.apps.v1.Deployment"),
            ("StatefulSet", "k8s_io.apps.v1.StatefulSet"),
            ("DaemonSet", "k8s_io.apps.v1.DaemonSet"),
            ("ReplicaSet", "k8s_io.apps.v1.ReplicaSet"),
            // Batch types
            ("Job", "k8s_io.batch.v1.Job"),
            ("CronJob", "k8s_io.batch.v1.CronJob"),
            // Networking
            ("Ingress", "k8s_io.networking.v1.Ingress"),
            ("NetworkPolicy", "k8s_io.networking.v1.NetworkPolicy"),
            // RBAC
            ("Role", "k8s_io.rbac.v1.Role"),
            ("ClusterRole", "k8s_io.rbac.v1.ClusterRole"),
            ("RoleBinding", "k8s_io.rbac.v1.RoleBinding"),
            ("ClusterRoleBinding", "k8s_io.rbac.v1.ClusterRoleBinding"),
        ];

        for (name, path) in shortcuts {
            writeln!(output, "  {} = {},", name, path).unwrap();
        }
    }

    /// Generate the helpers.ncl file content
    pub fn generate_helpers() -> String {
        r#"# Helper functions for Kubernetes resource construction
# Generated by Amalgam

{
  # Metadata constructor
  # Creates a standard K8s metadata object
  Metadata = fun { name, namespace ? "default", labels ? {}, annotations ? {} } =>
    {
      name = name,
      namespace = namespace,
      labels = labels,
      annotations = annotations,
    },

  # Environment variable constructors
  EnvVar = fun name value => { name = name, value = value },

  EnvFromSecret = fun name secretName key => {
    name = name,
    valueFrom.secretKeyRef = { name = secretName, key = key },
  },

  EnvFromConfigMap = fun name configMapName key => {
    name = name,
    valueFrom.configMapKeyRef = { name = configMapName, key = key },
  },

  EnvFromField = fun name fieldPath => {
    name = name,
    valueFrom.fieldRef = { fieldPath = fieldPath },
  },

  # Resource presets for containers
  Resources = {
    tiny = {
      requests = { cpu = "50m", memory = "64Mi" },
      limits = { cpu = "100m", memory = "128Mi" },
    },
    small = {
      requests = { cpu = "100m", memory = "128Mi" },
      limits = { cpu = "200m", memory = "256Mi" },
    },
    medium = {
      requests = { cpu = "250m", memory = "256Mi" },
      limits = { cpu = "500m", memory = "512Mi" },
    },
    large = {
      requests = { cpu = "500m", memory = "512Mi" },
      limits = { cpu = "1000m", memory = "1Gi" },
    },
    xlarge = {
      requests = { cpu = "1000m", memory = "1Gi" },
      limits = { cpu = "2000m", memory = "2Gi" },
    },
  },

  # Container port constructor
  ContainerPort = fun { port, name ? null, protocol ? "TCP" } =>
    {
      containerPort = port,
      protocol = protocol,
    } & (if name != null then { name = name } else {}),

  # Service port constructor
  ServicePort = fun { port, targetPort ? port, name ? null, protocol ? "TCP" } =>
    {
      port = port,
      targetPort = targetPort,
      protocol = protocol,
    } & (if name != null then { name = name } else {}),

  # Volume mount constructor
  VolumeMount = fun { name, mountPath, readOnly ? false, subPath ? null } =>
    {
      name = name,
      mountPath = mountPath,
      readOnly = readOnly,
    } & (if subPath != null then { subPath = subPath } else {}),

  # Common label sets
  Labels = {
    # Standard app labels
    app = fun name => { "app.kubernetes.io/name" = name },

    # Full app labels
    fullApp = fun { name, version ? "latest", component ? null, partOf ? null } =>
      {
        "app.kubernetes.io/name" = name,
        "app.kubernetes.io/version" = version,
      }
      & (if component != null then { "app.kubernetes.io/component" = component } else {})
      & (if partOf != null then { "app.kubernetes.io/part-of" = partOf } else {}),
  },

  # Probe constructors for health checks
  Probes = {
    # HTTP GET probe
    httpGet = fun { path, port, initialDelaySeconds ? 10, periodSeconds ? 10 } =>
      {
        httpGet = { path = path, port = port },
        initialDelaySeconds = initialDelaySeconds,
        periodSeconds = periodSeconds,
      },

    # TCP socket probe
    tcpSocket = fun { port, initialDelaySeconds ? 10, periodSeconds ? 10 } =>
      {
        tcpSocket = { port = port },
        initialDelaySeconds = initialDelaySeconds,
        periodSeconds = periodSeconds,
      },

    # Exec probe
    exec = fun { command, initialDelaySeconds ? 10, periodSeconds ? 10 } =>
      {
        exec = { command = command },
        initialDelaySeconds = initialDelaySeconds,
        periodSeconds = periodSeconds,
      },
  },
}
"#
        .to_string()
    }

    /// Generate the mixins.ncl file content
    pub fn generate_mixins() -> String {
        r#"# Mixins for common K8s patterns
# Generated by Amalgam
#
# Usage:
#   let deployment = k8s.Deployment & mixins.WithHealthChecks { path = "/health", port = 8080 }

{
  # Add health check probes to a container
  WithHealthChecks = fun { path, port, initialDelay ? 10, period ? 10 } =>
    {
      spec.template.spec.containers | std.array.map (fun container =>
        container & {
          livenessProbe = {
            httpGet = { path = path, port = port },
            initialDelaySeconds = initialDelay,
            periodSeconds = period,
          },
          readinessProbe = {
            httpGet = { path = path, port = port },
            initialDelaySeconds = initialDelay,
            periodSeconds = period,
          },
        }
      ),
    },

  # Add resource limits and requests
  WithResources = fun { requests, limits ? requests } =>
    {
      spec.template.spec.containers | std.array.map (fun container =>
        container & {
          resources = {
            requests = requests,
            limits = limits,
          },
        }
      ),
    },

  # Add common security context
  WithSecurityContext = fun { runAsNonRoot ? true, readOnlyRootFilesystem ? true } =>
    {
      spec.template.spec.securityContext = {
        runAsNonRoot = runAsNonRoot,
      },
      spec.template.spec.containers | std.array.map (fun container =>
        container & {
          securityContext = {
            readOnlyRootFilesystem = readOnlyRootFilesystem,
            allowPrivilegeEscalation = false,
          },
        }
      ),
    },

  # Add environment variables from a config map
  WithConfigMapEnv = fun configMapName =>
    {
      spec.template.spec.containers | std.array.map (fun container =>
        container & {
          envFrom = (container.envFrom or []) @ [{
            configMapRef = { name = configMapName },
          }],
        }
      ),
    },

  # Add environment variables from a secret
  WithSecretEnv = fun secretName =>
    {
      spec.template.spec.containers | std.array.map (fun container =>
        container & {
          envFrom = (container.envFrom or []) @ [{
            secretRef = { name = secretName },
          }],
        }
      ),
    },

  # Add node selector
  WithNodeSelector = fun selector =>
    {
      spec.template.spec.nodeSelector = selector,
    },

  # Add tolerations
  WithTolerations = fun tolerations =>
    {
      spec.template.spec.tolerations = tolerations,
    },

  # Add affinity rules
  WithAffinity = fun affinity =>
    {
      spec.template.spec.affinity = affinity,
    },

  # Add service account
  WithServiceAccount = fun serviceAccountName =>
    {
      spec.template.spec.serviceAccountName = serviceAccountName,
    },

  # Add image pull secrets
  WithImagePullSecrets = fun secrets =>
    {
      spec.template.spec.imagePullSecrets = secrets | std.array.map (fun name => { name = name }),
    },

  # Add labels to pod template
  WithPodLabels = fun labels =>
    {
      spec.template.metadata.labels = (labels),
    },

  # Add annotations to pod template
  WithPodAnnotations = fun annotations =>
    {
      spec.template.metadata.annotations = annotations,
    },

  # Configure horizontal pod autoscaler compatibility
  WithHPALabels = fun { minReplicas, maxReplicas, targetCPUUtilization ? 80 } =>
    {
      metadata.annotations = {
        "autoscaling.kubernetes.io/minReplicas" = std.to_string minReplicas,
        "autoscaling.kubernetes.io/maxReplicas" = std.to_string maxReplicas,
        "autoscaling.kubernetes.io/targetCPUUtilization" = std.to_string targetCPUUtilization,
      },
    },
}
"#
        .to_string()
    }

    /// Write the prelude and supporting files to a directory
    pub fn write_to_directory(&self, dir: &Path) -> std::io::Result<()> {
        // Write prelude.ncl
        let prelude_path = dir.join(&self.config.filename);
        std::fs::write(&prelude_path, self.generate())?;

        // Create lib directory if helpers or mixins are enabled
        if self.config.generate_helpers || self.config.generate_mixins {
            let lib_dir = dir.join("lib");
            std::fs::create_dir_all(&lib_dir)?;

            if self.config.generate_helpers {
                std::fs::write(lib_dir.join("helpers.ncl"), Self::generate_helpers())?;
            }

            if self.config.generate_mixins {
                std::fs::write(lib_dir.join("mixins.ncl"), Self::generate_mixins())?;
            }
        }

        Ok(())
    }
}

/// Convenience function to generate a prelude from a package directory
pub fn generate_prelude_for_directory(root: &Path) -> String {
    let packages = PreludeGenerator::discover_packages(root);
    let config = PreludeConfig {
        packages,
        ..Default::default()
    };
    let generator = PreludeGenerator::new(config);
    generator.generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude_generation_empty() {
        let config = PreludeConfig::default();
        let generator = PreludeGenerator::new(config);
        let output = generator.generate();

        assert!(output.contains("Generated prelude.ncl"));
        assert!(output.contains("helpers = import \"./lib/helpers.ncl\""));
        assert!(output.contains("mixins = import \"./lib/mixins.ncl\""));
    }

    #[test]
    fn test_prelude_with_package() {
        let config = PreludeConfig {
            packages: vec![PackageEntry {
                name: "k8s_io".to_string(),
                path: PathBuf::from("./k8s_io"),
                api_groups: vec![
                    ApiGroup {
                        name: "core".to_string(),
                        versions: vec!["v1".to_string()],
                        path: PathBuf::from("api/core"),
                    },
                    ApiGroup {
                        name: "apps".to_string(),
                        versions: vec!["v1".to_string()],
                        path: PathBuf::from("api/apps"),
                    },
                ],
            }],
            k8s_shortcuts: true,
            ..Default::default()
        };

        let generator = PreludeGenerator::new(config);
        let output = generator.generate();

        assert!(output.contains("k8s_io = {"));
        assert!(output.contains("core.v1 = import \"./k8s_io/api/core/v1.ncl\""));
        assert!(output.contains("apps.v1 = import \"./k8s_io/api/apps/v1.ncl\""));
        assert!(output.contains("Deployment = k8s_io.apps.v1.Deployment"));
        assert!(output.contains("Pod = k8s_io.core.v1.Pod"));
    }

    #[test]
    fn test_helpers_generation() {
        let helpers = PreludeGenerator::generate_helpers();

        assert!(helpers.contains("Metadata = fun"));
        assert!(helpers.contains("EnvVar = fun"));
        assert!(helpers.contains("Resources = {"));
        assert!(helpers.contains("tiny ="));
        assert!(helpers.contains("Probes = {"));
    }

    #[test]
    fn test_mixins_generation() {
        let mixins = PreludeGenerator::generate_mixins();

        assert!(mixins.contains("WithHealthChecks = fun"));
        assert!(mixins.contains("WithResources = fun"));
        assert!(mixins.contains("WithSecurityContext = fun"));
        assert!(mixins.contains("WithServiceAccount = fun"));
    }
}
