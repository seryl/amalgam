use amalgam_parser::swagger::parse_swagger_json;

#[tokio::test]
async fn test_swagger_has_authentication_modules() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the swagger JSON
    let url = "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.33.4/api/openapi-spec/swagger.json";
    let content = reqwest::get(url).await?.text().await?;

    // Parse it
    let ir = parse_swagger_json(&content)?;

    // Check for authentication modules
    let mut auth_count = 0;
    let mut discovery_count = 0;

    println!("Modules from swagger parser:");
    for module in &ir.modules {
        println!(
            "  Module: {} with {} types",
            module.name,
            module.types.len()
        );
        if module.name.contains("authentication") {
            auth_count += 1;
            println!("    ^ AUTHENTICATION MODULE!");
            // Print some types
            for (i, ty) in module.types.iter().enumerate() {
                if i < 5 {
                    println!("      - {}", ty.name);
                }
            }
        } else if module.name.contains("discovery") {
            discovery_count += 1;
            println!("    ^ DISCOVERY MODULE!");
        }
    }

    println!("\nSummary:");
    println!("  Total modules: {}", ir.modules.len());
    println!("  Authentication modules: {}", auth_count);
    println!("  Discovery modules: {}", discovery_count);

    // Assert we have authentication modules
    assert!(
        auth_count > 0,
        "Should have authentication modules from swagger"
    );
    assert!(
        discovery_count > 0,
        "Should have discovery modules from swagger"
    );

    Ok(())
}
