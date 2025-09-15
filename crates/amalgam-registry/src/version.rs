//! Version constraint parsing and matching

use anyhow::Result;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Version constraint for dependency resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionConstraint {
    /// Any version matches
    Any,
    /// Exact version match
    Exact(String),
    /// Semver requirement (e.g., "^1.0", ">=2.0 <3.0")
    Requirement(String),
    /// Complex range constraint
    Range(VersionRange),
}

impl VersionConstraint {
    /// Parse a version constraint string
    pub fn parse(input: &str) -> Result<Self> {
        if input == "*" || input.is_empty() {
            return Ok(Self::Any);
        }

        // Check for exact version (starts with =)
        if let Some(version) = input.strip_prefix('=') {
            return Ok(Self::Exact(version.to_string()));
        }

        // Check for complex range
        if input.contains("||") || (input.contains(">=") && input.contains("<")) {
            return Ok(Self::Range(VersionRange::parse(input)?));
        }

        // Default to semver requirement
        Ok(Self::Requirement(input.to_string()))
    }

    /// Check if a version matches this constraint
    pub fn matches(&self, version: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(v) => v == version,
            Self::Requirement(req) => match (VersionReq::parse(req), Version::parse(version)) {
                (Ok(req), Ok(ver)) => req.matches(&ver),
                _ => false,
            },
            Self::Range(range) => range.matches(version),
        }
    }

    /// Get the minimum version that satisfies this constraint
    pub fn minimum_version(&self) -> Option<String> {
        match self {
            Self::Any => Some("0.0.0".to_string()),
            Self::Exact(v) => Some(v.clone()),
            Self::Requirement(req) => {
                // Parse common patterns
                if let Some(v) = req.strip_prefix('^') {
                    Some(v.to_string())
                } else if let Some(v) = req.strip_prefix('~') {
                    Some(v.to_string())
                } else {
                    req.strip_prefix(">=").map(|v| v.trim().to_string())
                }
            }
            Self::Range(range) => range.minimum_version(),
        }
    }
}

impl fmt::Display for VersionConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => write!(f, "*"),
            Self::Exact(v) => write!(f, "={}", v),
            Self::Requirement(req) => write!(f, "{}", req),
            Self::Range(range) => write!(f, "{}", range),
        }
    }
}

/// Complex version range constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRange {
    pub constraints: Vec<RangeConstraint>,
}

impl VersionRange {
    /// Parse a complex version range
    pub fn parse(input: &str) -> Result<Self> {
        let mut constraints = Vec::new();

        // Split by OR operator
        for part in input.split("||") {
            let part = part.trim();

            // Check for AND constraints (space or comma separated)
            if part.contains(">=") && part.contains("<") {
                // Parse as bounded range
                let parts: Vec<&str> = part.split_whitespace().collect();
                if parts.len() >= 2 {
                    let min = parts[0]
                        .strip_prefix(">=")
                        .ok_or_else(|| anyhow::anyhow!("Invalid range: {}", part))?;
                    let max = parts[1]
                        .strip_prefix("<")
                        .ok_or_else(|| anyhow::anyhow!("Invalid range: {}", part))?;

                    constraints.push(RangeConstraint::Bounded {
                        min: min.to_string(),
                        max: max.to_string(),
                        min_inclusive: true,
                        max_inclusive: false,
                    });
                }
            } else if let Some(min) = part.strip_prefix(">=") {
                constraints.push(RangeConstraint::Minimum {
                    version: min.trim().to_string(),
                    inclusive: true,
                });
            } else if let Some(min) = part.strip_prefix('>') {
                constraints.push(RangeConstraint::Minimum {
                    version: min.trim().to_string(),
                    inclusive: false,
                });
            } else if let Some(max) = part.strip_prefix("<=") {
                constraints.push(RangeConstraint::Maximum {
                    version: max.trim().to_string(),
                    inclusive: true,
                });
            } else if let Some(max) = part.strip_prefix('<') {
                constraints.push(RangeConstraint::Maximum {
                    version: max.trim().to_string(),
                    inclusive: false,
                });
            } else {
                // Try as semver requirement
                constraints.push(RangeConstraint::Requirement(part.to_string()));
            }
        }

        Ok(Self { constraints })
    }

    /// Check if a version matches any constraint in the range
    pub fn matches(&self, version: &str) -> bool {
        let ver = match Version::parse(version) {
            Ok(v) => v,
            Err(_) => return false,
        };

        self.constraints.iter().any(|c| c.matches(&ver))
    }

    /// Get the minimum version from the range
    pub fn minimum_version(&self) -> Option<String> {
        self.constraints
            .iter()
            .filter_map(|c| c.minimum_version())
            .min()
    }
}

impl fmt::Display for VersionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.constraints.iter().map(|c| c.to_string()).collect();
        write!(f, "{}", parts.join(" || "))
    }
}

/// Individual range constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RangeConstraint {
    Minimum {
        version: String,
        inclusive: bool,
    },
    Maximum {
        version: String,
        inclusive: bool,
    },
    Bounded {
        min: String,
        max: String,
        min_inclusive: bool,
        max_inclusive: bool,
    },
    Requirement(String),
}

impl RangeConstraint {
    /// Check if a version matches this constraint
    pub fn matches(&self, version: &Version) -> bool {
        match self {
            Self::Minimum {
                version: min,
                inclusive,
            } => match Version::parse(min) {
                Ok(min_ver) => {
                    if *inclusive {
                        version >= &min_ver
                    } else {
                        version > &min_ver
                    }
                }
                Err(_) => false,
            },
            Self::Maximum {
                version: max,
                inclusive,
            } => match Version::parse(max) {
                Ok(max_ver) => {
                    if *inclusive {
                        version <= &max_ver
                    } else {
                        version < &max_ver
                    }
                }
                Err(_) => false,
            },
            Self::Bounded {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => match (Version::parse(min), Version::parse(max)) {
                (Ok(min_ver), Ok(max_ver)) => {
                    let min_ok = if *min_inclusive {
                        version >= &min_ver
                    } else {
                        version > &min_ver
                    };
                    let max_ok = if *max_inclusive {
                        version <= &max_ver
                    } else {
                        version < &max_ver
                    };
                    min_ok && max_ok
                }
                _ => false,
            },
            Self::Requirement(req) => match VersionReq::parse(req) {
                Ok(req) => req.matches(version),
                Err(_) => false,
            },
        }
    }

    /// Get the minimum version for this constraint
    pub fn minimum_version(&self) -> Option<String> {
        match self {
            Self::Minimum { version, .. } => Some(version.clone()),
            Self::Bounded { min, .. } => Some(min.clone()),
            Self::Requirement(req) => {
                if let Some(v) = req.strip_prefix('^') {
                    Some(v.to_string())
                } else {
                    req.strip_prefix('~').map(|v| v.to_string())
                }
            }
            _ => None,
        }
    }
}

impl fmt::Display for RangeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minimum { version, inclusive } => {
                write!(f, "{}{}", if *inclusive { ">=" } else { ">" }, version)
            }
            Self::Maximum { version, inclusive } => {
                write!(f, "{}{}", if *inclusive { "<=" } else { "<" }, version)
            }
            Self::Bounded {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                write!(
                    f,
                    "{}{} {}{}",
                    if *min_inclusive { ">=" } else { ">" },
                    min,
                    if *max_inclusive { "<=" } else { "<" },
                    max
                )
            }
            Self::Requirement(req) => write!(f, "{}", req),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constraint_parsing() {
        // Any version
        let constraint = VersionConstraint::parse("*").unwrap();
        assert!(constraint.matches("1.0.0"));
        assert!(constraint.matches("2.3.4"));

        // Exact version
        let constraint = VersionConstraint::parse("=1.0.0").unwrap();
        assert!(constraint.matches("1.0.0"));
        assert!(!constraint.matches("1.0.1"));

        // Caret requirement
        let constraint = VersionConstraint::parse("^1.0.0").unwrap();
        assert!(constraint.matches("1.0.0"));
        assert!(constraint.matches("1.5.0"));
        assert!(!constraint.matches("2.0.0"));

        // Range
        let constraint = VersionConstraint::parse(">=1.0.0 <2.0.0").unwrap();
        assert!(constraint.matches("1.0.0"));
        assert!(constraint.matches("1.9.9"));
        assert!(!constraint.matches("2.0.0"));
    }

    #[test]
    fn test_complex_range() {
        let range = VersionRange::parse(">=1.0.0 <2.0.0 || >=3.0.0 <4.0.0").unwrap();

        assert!(range.matches("1.5.0"));
        assert!(!range.matches("2.5.0"));
        assert!(range.matches("3.5.0"));
        assert!(!range.matches("4.0.0"));
    }

    #[test]
    fn test_minimum_version() {
        let constraint = VersionConstraint::parse("^1.2.3").unwrap();
        assert_eq!(constraint.minimum_version(), Some("1.2.3".to_string()));

        let constraint = VersionConstraint::parse(">=2.0.0").unwrap();
        assert_eq!(constraint.minimum_version(), Some("2.0.0".to_string()));
    }
}
