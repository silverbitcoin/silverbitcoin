//! # Dependency Management
//!
//! Dependency resolution and installation.

use crate::manifest::{Dependency, DetailedDependency, Manifest};
use crate::registry::Registry;
use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

/// Dependency resolver
pub struct DependencyResolver {
    registry: Registry,
    cache_dir: PathBuf,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new(registry_url: Option<&str>) -> Result<Self> {
        let registry = Registry::new(registry_url)?;
        
        // Get cache directory
        let cache_dir = get_cache_dir()?;
        std::fs::create_dir_all(&cache_dir)?;
        
        Ok(Self {
            registry,
            cache_dir,
        })
    }
    
    /// Resolve all dependencies for a manifest
    pub async fn resolve(&self, manifest: &Manifest) -> Result<ResolvedDependencies> {
        let mut resolved = ResolvedDependencies::new();
        let mut to_resolve = VecDeque::new();
        
        // Add direct dependencies
        for (name, dep) in &manifest.dependencies {
            to_resolve.push_back((name.clone(), dep.clone(), 0));
        }
        
        // Resolve dependencies recursively
        while let Some((name, dep, depth)) = to_resolve.pop_front() {
            if depth > 100 {
                anyhow::bail!("Dependency depth limit exceeded (possible circular dependency)");
            }
            
            if resolved.contains(&name) {
                continue;
            }
            
            let dep_info = self.resolve_single(&name, &dep).await?;
            
            // Add transitive dependencies
            for (trans_name, trans_dep) in &dep_info.manifest.dependencies {
                to_resolve.push_back((trans_name.clone(), trans_dep.clone(), depth + 1));
            }
            
            resolved.add(name, dep_info);
        }
        
        Ok(resolved)
    }
    
    /// Resolve a single dependency
    async fn resolve_single(&self, name: &str, dep: &Dependency) -> Result<DependencyInfo> {
        match dep {
            Dependency::Simple(version) => {
                self.resolve_registry_dependency(name, version).await
            }
            Dependency::Detailed(detailed) => {
                if let Some(path) = &detailed.path {
                    self.resolve_path_dependency(name, path)
                } else if let Some(git) = &detailed.git {
                    self.resolve_git_dependency(name, git, detailed).await
                } else if let Some(version) = &detailed.version {
                    self.resolve_registry_dependency(name, version).await
                } else {
                    anyhow::bail!("Invalid dependency specification for {}", name)
                }
            }
        }
    }
    
    /// Resolve a registry dependency
    async fn resolve_registry_dependency(&self, name: &str, version: &str) -> Result<DependencyInfo> {
        // Check cache first
        let cache_path = self.cache_dir.join(format!("{}-{}", name, version));
        
        if cache_path.exists() {
            return self.load_cached_dependency(&cache_path);
        }
        
        // Download from registry
        let archive = self.registry.download(name, version).await?;
        
        // Extract to cache
        extract_archive(&archive, &cache_path)?;
        
        self.load_cached_dependency(&cache_path)
    }
    
    /// Resolve a path dependency
    fn resolve_path_dependency(&self, name: &str, path: &str) -> Result<DependencyInfo> {
        let dep_path = PathBuf::from(path);
        
        if !dep_path.exists() {
            anyhow::bail!("Path dependency not found: {}", path);
        }
        
        let manifest_path = dep_path.join("Quantum.toml");
        let manifest = Manifest::load(&manifest_path)?;
        
        Ok(DependencyInfo {
            name: name.to_string(),
            version: manifest.package.version.clone(),
            path: dep_path,
            manifest,
            source: DependencySource::Path,
        })
    }
    
    /// Resolve a git dependency from a remote repository.
    ///
    /// Clones the repository, checks out the specified ref, and loads the manifest.
    ///
    /// # Arguments
    /// * `_name` - The dependency name
    /// * `git_url` - The git repository URL
    /// * `detailed` - Detailed dependency specification with branch/tag/rev
    async fn resolve_git_dependency(
        &self,
        _name: &str,
        git_url: &str,
        detailed: &DetailedDependency,
    ) -> Result<DependencyInfo> {
        // Create cache key from git URL and ref
        let ref_str = detailed.branch.as_deref()
            .or(detailed.tag.as_deref())
            .or(detailed.rev.as_deref())
            .unwrap_or("HEAD");
        
        let cache_key = format!("{}-{}", 
            git_url.replace(['/', ':'], "_"),
            ref_str.replace('/', "_")
        );
        
        let cache_path = self.cache_dir.join(cache_key);
        
        if cache_path.exists() {
            return self.load_cached_dependency(&cache_path);
        }
        
        // Clone repository
        clone_git_repo(git_url, &cache_path, detailed)?;
        
        self.load_cached_dependency(&cache_path)
    }
    
    /// Load dependency from cache
    fn load_cached_dependency(&self, path: &Path) -> Result<DependencyInfo> {
        let manifest_path = path.join("Quantum.toml");
        let manifest = Manifest::load(&manifest_path)?;
        
        Ok(DependencyInfo {
            name: manifest.package.name.clone(),
            version: manifest.package.version.clone(),
            path: path.to_path_buf(),
            manifest,
            source: DependencySource::Registry,
        })
    }
}

/// Resolved dependencies
pub struct ResolvedDependencies {
    dependencies: HashMap<String, DependencyInfo>,
}

impl ResolvedDependencies {
    fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
        }
    }
    
    fn contains(&self, name: &str) -> bool {
        self.dependencies.contains_key(name)
    }
    
    fn add(&mut self, name: String, info: DependencyInfo) {
        self.dependencies.insert(name, info);
    }
    
    /// Get all dependencies
    pub fn all(&self) -> &HashMap<String, DependencyInfo> {
        &self.dependencies
    }
    
    /// Get dependency by name.
    ///
    /// # Arguments
    /// * `name` - The dependency name to look up
    ///
    /// # Returns
    /// A reference to the dependency info if found
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&DependencyInfo> {
        self.dependencies.get(name)
    }
}

/// Information about a resolved dependency.
///
/// Contains all metadata about a dependency including:
/// - Name and version
/// - Local path where it's stored
/// - Manifest with configuration
/// - Source (registry, path, or git)
pub struct DependencyInfo {
    /// The dependency name
    pub name: String,
    /// The dependency version
    pub version: String,
    /// The local path where the dependency is stored
    #[allow(dead_code)]
    pub path: PathBuf,
    /// The dependency's manifest
    pub manifest: Manifest,
    /// The source of the dependency
    pub source: DependencySource,
}

/// Dependency source indicating where a dependency comes from.
///
/// Specifies the origin of a dependency:
/// - Registry: Downloaded from the package registry
/// - Path: Local filesystem path
/// - Git: Remote git repository
pub enum DependencySource {
    /// Dependency from the package registry
    Registry,
    /// Dependency from a local filesystem path
    Path,
    /// Dependency from a git repository
    #[allow(dead_code)]
    Git,
}

/// Get cache directory
fn get_cache_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("Failed to get home directory")?;
    
    Ok(PathBuf::from(home).join(".quantum").join("cache"))
}

/// Extract tar archive
fn extract_archive(archive: &[u8], dest: &Path) -> Result<()> {
    use std::io::Cursor;
    
    std::fs::create_dir_all(dest)?;
    
    let cursor = Cursor::new(archive);
    let mut tar = tar::Archive::new(cursor);
    
    tar.unpack(dest)?;
    
    Ok(())
}

/// Clone git repository
fn clone_git_repo(url: &str, dest: &Path, detailed: &DetailedDependency) -> Result<()> {
    use std::process::Command;
    
    // Clone repository
    let mut cmd = Command::new("git");
    cmd.arg("clone");
    
    if let Some(branch) = &detailed.branch {
        cmd.arg("--branch").arg(branch);
    }
    
    cmd.arg(url).arg(dest);
    
    let output = cmd.output()
        .context("Failed to execute git clone")?;
    
    if !output.status.success() {
        anyhow::bail!("Git clone failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Checkout specific ref if specified
    if let Some(rev) = &detailed.rev {
        let output = Command::new("git")
            .arg("-C")
            .arg(dest)
            .arg("checkout")
            .arg(rev)
            .output()
            .context("Failed to checkout git revision")?;
        
        if !output.status.success() {
            anyhow::bail!("Git checkout failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
    
    Ok(())
}
