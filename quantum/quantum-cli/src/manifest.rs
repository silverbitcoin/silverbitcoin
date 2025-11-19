//! # Quantum Package Manifest
//!
//! Handles parsing and manipulation of Quantum.toml manifest files.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};

/// Package manifest (Quantum.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Package metadata
    pub package: PackageMetadata,
    /// Dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
    /// Dev dependencies
    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: HashMap<String, Dependency>,
    /// Build configuration
    #[serde(default)]
    pub build: BuildConfig,
}

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package authors
    #[serde(default)]
    pub authors: Vec<String>,
    /// Package description
    #[serde(default)]
    pub description: Option<String>,
    /// Package license
    #[serde(default)]
    pub license: Option<String>,
    /// Repository URL
    #[serde(default)]
    pub repository: Option<String>,
    /// Homepage URL
    #[serde(default)]
    pub homepage: Option<String>,
    /// Package edition (e.g., "2024")
    #[serde(default = "default_edition")]
    pub edition: String,
}

fn default_edition() -> String {
    "2024".to_string()
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Simple version string
    Simple(String),
    /// Detailed dependency specification
    Detailed(DetailedDependency),
}

/// Detailed dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedDependency {
    /// Version requirement
    #[serde(default)]
    pub version: Option<String>,
    /// Git repository URL
    #[serde(default)]
    pub git: Option<String>,
    /// Git branch
    #[serde(default)]
    pub branch: Option<String>,
    /// Git tag
    #[serde(default)]
    pub tag: Option<String>,
    /// Git revision
    #[serde(default)]
    pub rev: Option<String>,
    /// Local path
    #[serde(default)]
    pub path: Option<String>,
    /// Registry URL
    #[serde(default)]
    pub registry: Option<String>,
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildConfig {
    /// Optimization level (0-3)
    #[serde(default = "default_opt_level")]
    pub opt_level: u8,
    /// Enable debug info
    #[serde(default)]
    pub debug: bool,
    /// Target address size (32 or 64)
    #[serde(default = "default_address_size")]
    pub address_size: u8,
}

fn default_opt_level() -> u8 {
    2
}

fn default_address_size() -> u8 {
    64
}

impl Manifest {
    /// Load manifest from Quantum.toml file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .context("Failed to read Quantum.toml")?;
        
        let manifest: Manifest = toml::from_str(&content)
            .context("Failed to parse Quantum.toml")?;
        
        manifest.validate()?;
        
        Ok(manifest)
    }
    
    /// Save manifest to Quantum.toml file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize manifest")?;
        
        std::fs::write(path.as_ref(), content)
            .context("Failed to write Quantum.toml")?;
        
        Ok(())
    }
    
    /// Create a new manifest with default values
    pub fn new(name: String) -> Self {
        Self {
            package: PackageMetadata {
                name,
                version: "0.1.0".to_string(),
                authors: vec![],
                description: None,
                license: None,
                repository: None,
                homepage: None,
                edition: "2024".to_string(),
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build: BuildConfig::default(),
        }
    }
    
    /// Validate manifest
    fn validate(&self) -> Result<()> {
        // Validate package name
        if self.package.name.is_empty() {
            anyhow::bail!("Package name cannot be empty");
        }
        
        if !self.package.name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            anyhow::bail!("Package name can only contain alphanumeric characters, underscores, and hyphens");
        }
        
        // Validate version
        if !is_valid_version(&self.package.version) {
            anyhow::bail!("Invalid version format: {}", self.package.version);
        }
        
        // Validate edition
        if !["2024"].contains(&self.package.edition.as_str()) {
            anyhow::bail!("Unsupported edition: {}", self.package.edition);
        }
        
        // Validate build config
        if self.build.opt_level > 3 {
            anyhow::bail!("Optimization level must be 0-3");
        }
        
        if self.build.address_size != 32 && self.build.address_size != 64 {
            anyhow::bail!("Address size must be 32 or 64");
        }
        
        Ok(())
    }
    
    /// Get all dependencies (including dev dependencies).
    ///
    /// Returns a map of all dependencies and dev dependencies combined.
    ///
    /// # Returns
    /// A HashMap containing all dependencies with their names as keys
    #[allow(dead_code)]
    pub fn all_dependencies(&self) -> HashMap<String, &Dependency> {
        let mut deps = HashMap::new();
        
        for (name, dep) in &self.dependencies {
            deps.insert(name.clone(), dep);
        }
        
        for (name, dep) in &self.dev_dependencies {
            deps.insert(name.clone(), dep);
        }
        
        deps
    }
}

/// Check if version string is valid (semver)
fn is_valid_version(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    
    if parts.len() != 3 {
        return false;
    }
    
    parts.iter().all(|part| part.parse::<u32>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_manifest_creation() {
        let manifest = Manifest::new("test_package".to_string());
        assert_eq!(manifest.package.name, "test_package");
        assert_eq!(manifest.package.version, "0.1.0");
        assert_eq!(manifest.package.edition, "2024");
    }
    
    #[test]
    fn test_version_validation() {
        assert!(is_valid_version("0.1.0"));
        assert!(is_valid_version("1.2.3"));
        assert!(!is_valid_version("1.2"));
        assert!(!is_valid_version("1.2.3.4"));
        assert!(!is_valid_version("abc"));
    }
}
