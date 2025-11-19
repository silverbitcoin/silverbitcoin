//! # New Command
//!
//! Create a new Quantum package.

use crate::package;
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::PathBuf;

/// Execute the `quantum new` command
pub async fn execute(name: &str, here: bool) -> Result<()> {
    // Validate package name
    if name.is_empty() {
        anyhow::bail!("Package name cannot be empty");
    }
    
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        anyhow::bail!("Package name can only contain alphanumeric characters, underscores, and hyphens");
    }
    
    // Determine package path
    let package_path = if here {
        std::env::current_dir()
            .context("Failed to get current directory")?
    } else {
        PathBuf::from(name)
    };
    
    // Check if directory already exists
    if package_path.exists() && !here {
        anyhow::bail!("Directory '{}' already exists", package_path.display());
    }
    
    // Create package
    println!("{} {} `{}`", "Creating".green().bold(), "Quantum package".bold(), name);
    
    let package = package::create_package(name, &package_path)
        .context("Failed to create package")?;
    
    println!("{} package structure created", "✓".green().bold());
    println!();
    println!("Package created at: {}", package.root.display());
    println!();
    println!("Next steps:");
    
    if !here {
        println!("  cd {}", name);
    }
    
    println!("  quantum build");
    println!("  quantum test");
    println!();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_new_command() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        execute("test_package", false).await.unwrap();
        
        let package_path = temp_dir.path().join("test_package");
        assert!(package_path.exists());
        assert!(package_path.join("Quantum.toml").exists());
        assert!(package_path.join("src/main.qm").exists());
    }
    
    #[tokio::test]
    async fn test_new_command_here() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        execute("test_package", true).await.unwrap();
        
        assert!(temp_dir.path().join("Quantum.toml").exists());
        assert!(temp_dir.path().join("src/main.qm").exists());
    }
}
