//! # Quantum Package Management
//!
//! Core package management functionality.

use crate::manifest::Manifest;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Quantum package structure
pub struct Package {
    /// Package root directory
    pub root: PathBuf,
    /// Package manifest
    pub manifest: Manifest,
}

impl Package {
    /// Load package from directory
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let root = path.as_ref().to_path_buf();
        let manifest_path = root.join("Quantum.toml");
        
        if !manifest_path.exists() {
            anyhow::bail!("Quantum.toml not found in {}", root.display());
        }
        
        let manifest = Manifest::load(&manifest_path)?;
        
        Ok(Self { root, manifest })
    }
    
    /// Load package from current directory
    pub fn load_current() -> Result<Self> {
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        
        Self::load(current_dir)
    }
    
    /// Get source directory
    pub fn src_dir(&self) -> PathBuf {
        self.root.join("src")
    }
    
    /// Get build directory
    pub fn build_dir(&self, release: bool) -> PathBuf {
        if release {
            self.root.join("build").join("release")
        } else {
            self.root.join("build").join("debug")
        }
    }
    
    /// Get all source files
    pub fn source_files(&self) -> Result<Vec<PathBuf>> {
        let src_dir = self.src_dir();
        
        if !src_dir.exists() {
            anyhow::bail!("Source directory not found: {}", src_dir.display());
        }
        
        let mut files = Vec::new();
        collect_quantum_files(&src_dir, &mut files)?;
        
        Ok(files)
    }
    
    /// Get package name
    pub fn name(&self) -> &str {
        &self.manifest.package.name
    }
    
    /// Get package version
    pub fn version(&self) -> &str {
        &self.manifest.package.version
    }
}

/// Recursively collect all .qm (Quantum) files
fn collect_quantum_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            collect_quantum_files(&path, files)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("qm") {
            files.push(path);
        }
    }
    
    Ok(())
}

/// Create a new package structure
pub fn create_package<P: AsRef<Path>>(name: &str, path: P) -> Result<Package> {
    let root = path.as_ref().to_path_buf();
    
    // Create directory structure
    std::fs::create_dir_all(&root)
        .context("Failed to create package directory")?;
    
    let src_dir = root.join("src");
    std::fs::create_dir_all(&src_dir)
        .context("Failed to create src directory")?;
    
    // Create manifest
    let manifest = Manifest::new(name.to_string());
    let manifest_path = root.join("Quantum.toml");
    manifest.save(&manifest_path)?;
    
    // Create main.qm file
    let main_file = src_dir.join("main.qm");
    let main_content = format!(
        r#"// {} - Main module
//
// This is the entry point for your Quantum smart contract.

module {}::main {{
    use silver::object::{{Self, UID}};
    use silver::transfer;
    use silver::tx_context::{{Self, TxContext}};

    /// Example object
    struct ExampleObject has key, store {{
        id: UID,
        value: u64,
    }}

    /// Create a new example object
    public fun create(value: u64, ctx: &mut TxContext): ExampleObject {{
        ExampleObject {{
            id: object::new(ctx),
            value,
        }}
    }}

    /// Transfer an example object
    public fun transfer_object(obj: ExampleObject, recipient: address) {{
        transfer::transfer(obj, recipient)
    }}

    /// Get the value of an example object
    public fun get_value(obj: &ExampleObject): u64 {{
        obj.value
    }}
}}
"#,
        name, name
    );
    
    std::fs::write(&main_file, main_content)
        .context("Failed to create main.qm")?;
    
    // Create .gitignore
    let gitignore_path = root.join(".gitignore");
    let gitignore_content = r#"/build
/target
*.swp
*.swo
*~
.DS_Store
"#;
    std::fs::write(&gitignore_path, gitignore_content)
        .context("Failed to create .gitignore")?;
    
    // Create README.md
    let readme_path = root.join("README.md");
    let readme_content = format!(
        r#"# {}

A Quantum smart contract package.

## Building

```bash
quantum build
```

## Testing

```bash
quantum test
```

## Publishing

```bash
quantum publish
```
"#,
        name
    );
    std::fs::write(&readme_path, readme_content)
        .context("Failed to create README.md")?;
    
    Ok(Package { root, manifest })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_create_package() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test_package");
        
        let package = create_package("test_package", &package_path).unwrap();
        
        assert_eq!(package.name(), "test_package");
        assert_eq!(package.version(), "0.1.0");
        assert!(package.root.join("Quantum.toml").exists());
        assert!(package.root.join("src").exists());
        assert!(package.root.join("src/main.qm").exists());
        assert!(package.root.join(".gitignore").exists());
        assert!(package.root.join("README.md").exists());
    }
}
