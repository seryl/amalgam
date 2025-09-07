//! Shared naming utilities for consistent case conversion across the codebase.
//!
//! This module provides ONLY simple case conversion functions. The actual
//! type names with correct casing MUST come from the ModuleRegistry, which
//! has the authoritative information from the original schemas.
//!
//! These functions are used for:
//! - Converting already properly-cased PascalCase names to camelCase for variables
//! - Simple capitalization when we know the input is a single word
//! - Test utilities where we're working with mock data
//!
//! DO NOT use these functions to guess the proper casing of type names!
//! Always get the correct type name from the ModuleRegistry.

/// Simple PascalCase conversion - ONLY for single words or already properly cased names
/// 
/// WARNING: This function does NOT handle complex names like "CELDeviceSelector".
/// For actual type names, you MUST use the ModuleRegistry which has the correct
/// casing from the schemas.
/// 
/// This function ONLY:
/// - Capitalizes the first letter of a single word
/// - Preserves existing capitalization if mixed case is detected
/// 
/// # Examples
/// ```
/// use amalgam_core::naming::to_pascal_case;
/// assert_eq!(to_pascal_case("pod"), "Pod");
/// assert_eq!(to_pascal_case("Pod"), "Pod");
/// assert_eq!(to_pascal_case("ObjectMeta"), "ObjectMeta"); // Already correct
/// assert_eq!(to_pascal_case("CELDeviceSelector"), "CELDeviceSelector"); // Already correct
/// // WARNING: These will NOT work correctly:
/// assert_eq!(to_pascal_case("objectmeta"), "Objectmeta"); // Wrong! Should be "ObjectMeta"
/// assert_eq!(to_pascal_case("celdeviceselector"), "Celdeviceselector"); // Wrong! Should be "CELDeviceSelector"
/// ```
pub fn to_pascal_case(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    
    // If it already has mixed case, assume it's correct and preserve it
    if name.chars().any(|c| c.is_uppercase()) && name.chars().any(|c| c.is_lowercase()) {
        // Just ensure first letter is uppercase
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    } else {
        // Single word or all same case - just capitalize first letter
        // This will be WRONG for complex names - use ModuleRegistry instead!
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
        }
    }
}

/// Convert PascalCase to camelCase for import variable names
/// 
/// # Examples
/// ```
/// use amalgam_core::naming::to_camel_case;
/// assert_eq!(to_camel_case("Pod"), "pod");
/// assert_eq!(to_camel_case("ObjectMeta"), "objectMeta");
/// assert_eq!(to_camel_case("CELDeviceSelector"), "cELDeviceSelector");
/// ```
pub fn to_camel_case(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert snake_case to PascalCase
/// 
/// # Examples
/// ```
/// use amalgam_core::naming::snake_to_pascal_case;
/// assert_eq!(snake_to_pascal_case("object_meta"), "ObjectMeta");
/// assert_eq!(snake_to_pascal_case("pod_spec"), "PodSpec");
/// ```
pub fn snake_to_pascal_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Convert snake_case to camelCase
/// 
/// # Examples
/// ```
/// use amalgam_core::naming::snake_to_camel_case;
/// assert_eq!(snake_to_camel_case("object_meta"), "objectMeta");
/// assert_eq!(snake_to_camel_case("pod_spec"), "podSpec");
/// ```
pub fn snake_to_camel_case(name: &str) -> String {
    let mut parts = name.split('_');
    match parts.next() {
        None => String::new(),
        Some(first) => {
            let mut result = first.to_string();
            for part in parts {
                let mut chars = part.chars();
                if let Some(first_char) = chars.next() {
                    result.push_str(&first_char.to_uppercase().collect::<String>());
                    result.push_str(chars.as_str());
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        // Basic single word cases - these work correctly
        assert_eq!(to_pascal_case(""), "");
        assert_eq!(to_pascal_case("pod"), "Pod");
        assert_eq!(to_pascal_case("Pod"), "Pod");
        assert_eq!(to_pascal_case("namespace"), "Namespace");
        
        // Already properly cased - preserved correctly
        assert_eq!(to_pascal_case("ObjectMeta"), "ObjectMeta");
        assert_eq!(to_pascal_case("CELDeviceSelector"), "CELDeviceSelector");
        assert_eq!(to_pascal_case("PodSpec"), "PodSpec");
        assert_eq!(to_pascal_case("ManagedFieldsEntry"), "ManagedFieldsEntry");
        assert_eq!(to_pascal_case("RawExtension"), "RawExtension");
        assert_eq!(to_pascal_case("IntOrString"), "IntOrString");
        assert_eq!(to_pascal_case("CSIDriver"), "CSIDriver");
        assert_eq!(to_pascal_case("HTTPProxy"), "HTTPProxy");
        assert_eq!(to_pascal_case("DNSConfig"), "DNSConfig");
        
        // These demonstrate the LIMITATION - they don't work correctly
        // In production, these MUST come from ModuleRegistry
        assert_eq!(to_pascal_case("objectmeta"), "Objectmeta"); // WRONG - should be ObjectMeta
        assert_eq!(to_pascal_case("celdeviceselector"), "Celdeviceselector"); // WRONG - should be CELDeviceSelector
        assert_eq!(to_pascal_case("podspec"), "Podspec"); // WRONG - should be PodSpec
        assert_eq!(to_pascal_case("managedfieldsentry"), "Managedfieldsentry"); // WRONG - should be ManagedFieldsEntry
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case(""), "");
        assert_eq!(to_camel_case("Pod"), "pod");
        assert_eq!(to_camel_case("pod"), "pod");
        assert_eq!(to_camel_case("ObjectMeta"), "objectMeta");
        assert_eq!(to_camel_case("objectMeta"), "objectMeta");
        assert_eq!(to_camel_case("CELDeviceSelector"), "cELDeviceSelector");
    }

    #[test]
    fn test_snake_to_pascal_case() {
        assert_eq!(snake_to_pascal_case(""), "");
        assert_eq!(snake_to_pascal_case("pod"), "Pod");
        assert_eq!(snake_to_pascal_case("object_meta"), "ObjectMeta");
        assert_eq!(snake_to_pascal_case("pod_spec"), "PodSpec");
        assert_eq!(snake_to_pascal_case("managed_fields_entry"), "ManagedFieldsEntry");
    }

    #[test]
    fn test_snake_to_camel_case() {
        assert_eq!(snake_to_camel_case(""), "");
        assert_eq!(snake_to_camel_case("pod"), "pod");
        assert_eq!(snake_to_camel_case("object_meta"), "objectMeta");
        assert_eq!(snake_to_camel_case("pod_spec"), "podSpec");
        assert_eq!(snake_to_camel_case("managed_fields_entry"), "managedFieldsEntry");
    }
}