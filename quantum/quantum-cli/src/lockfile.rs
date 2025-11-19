//! # Lockfile Management
//!
//! Manages Quantum.lock file for dependency locking.

use crate::dependency::ResolvedDependencies;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Lockfile (Quantum.lock)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    /// Lockfile version
    pub version: u32,
    /// Locked dependencies
    pub dependencies: HashMap<String, LockedDependency>,
}

/// Locked dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedDependency {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Source type
    pub source: String,
    /// Source URL or path
    pub source_url: Option<String>,
    /// Checksum
    pub checksum: Option<String>,
}

impl Lockfile {
    /// Create a new lockfile
    pub fn new() -> Self {
        Self {
            version: 1,
            dependencies: HashMap::new(),
        }
    }
    
    /// Load lockfile from Quantum.lock.
    ///
    /// Reads and parses the lockfile containing resolved dependencies.
    ///
    /// # Arguments
    /// * `path` - Path to the Quantum.lock file
    ///
    /// # Returns
    /// The parsed lockfile
    #[allow(dead_code)]
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .context("Failed to read Quantum.lock")?;
        
        let lockfile: Lockfile = toml::from_str(&content)
            .context("Failed to parse Quantum.lock")?;
        
        Ok(lockfile)
    }
    
    /// Save lockfile to Quantum.lock
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize lockfile")?;
        
        std::fs::write(path.as_ref(), content)
            .context("Failed to write Quantum.lock")?;
        
        Ok(())
    }
    
    /// Create lockfile from resolved dependencies
    pub fn from_resolved(resolved: &ResolvedDependencies) -> Self {
        let mut lockfile = Self::new();
        
        for (name, info) in resolved.all() {
            let source = match info.source {
                crate::dependency::DependencySource::Registry => "registry",
                crate::dependency::DependencySource::Path => "path",
                crate::dependency::DependencySource::Git => "git",
            };
            
            lockfile.dependencies.insert(
                name.clone(),
                LockedDependency {
                    name: info.name.clone(),
                    version: info.version.clone(),
                    source: source.to_string(),
                    source_url: None,
                    checksum: None,
                },
            );
        }
        
        lockfile
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self::new()
    }
}
