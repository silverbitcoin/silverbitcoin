//! # Publish Command
//!
//! Publish a Quantum package to the registry.

use crate::package::Package;
use crate::registry::Registry;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Confirm;

/// Execute the `quantum publish` command
pub async fn execute(skip_confirm: bool, registry_url: Option<&str>) -> Result<()> {
    // Load package
    let package = Package::load_current()
        .context("Failed to load package. Make sure you're in a Quantum package directory.")?;
    
    println!("{} {} v{}", 
        "Publishing".green().bold(), 
        package.name().bold(), 
        package.version()
    );
    
    // Validate package before publishing
    validate_package(&package)?;
    
    // Confirm publication
    if !skip_confirm {
        let confirmed = Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to publish {} v{} to the registry?",
                package.name(),
                package.version()
            ))
            .default(false)
            .interact()?;
        
        if !confirmed {
            println!("Publication cancelled");
            return Ok(());
        }
    }
    
    // Connect to registry
    let registry = Registry::new(registry_url)?;
    
    println!("Connecting to registry...");
    
    // Build package before publishing
    println!("Building package...");
    crate::commands::build::execute(true, None).await?;
    
    // Package and upload
    println!("Packaging...");
    let package_data = create_package_archive(&package)?;
    
    println!("Uploading to registry...");
    registry.publish(&package, package_data).await?;
    
    println!();
    println!("{} Package published successfully!", "✓".green().bold());
    println!("  Package: {} v{}", package.name(), package.version());
    println!("  Registry: {}", registry.url());
    
    Ok(())
}

/// Validate package before publishing
fn validate_package(package: &Package) -> Result<()> {
    // Check required fields
    if package.manifest.package.description.is_none() {
        anyhow::bail!("Package description is required for publishing. Add 'description' to Quantum.toml");
    }
    
    if package.manifest.package.license.is_none() {
        anyhow::bail!("Package license is required for publishing. Add 'license' to Quantum.toml");
    }
    
    // Check source files exist
    let source_files = package.source_files()?;
    if source_files.is_empty() {
        anyhow::bail!("No source files found. Cannot publish empty package.");
    }
    
    println!("{} Package validation passed", "✓".green().bold());
    
    Ok(())
}

/// Create package archive for upload to registry.
///
/// Packages the Quantum module into a tar archive containing:
/// - Manifest file (Quantum.toml)
/// - Source code files
/// - Documentation
/// - Metadata
///
/// # Arguments
/// * `package` - The package to archive
///
/// # Returns
/// A vector of bytes containing the tar archive data
fn create_package_archive(package: &Package) -> Result<Vec<u8>> {
    let mut archive = Vec::new();
    let mut tar = tar::Builder::new(&mut archive);
    
    // Add manifest
    let manifest_content = toml::to_string_pretty(&package.manifest)?;
    let mut header = tar::Header::new_gnu();
    header.set_size(manifest_content.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append_data(&mut header, "Quantum.toml", manifest_content.as_bytes())?;
    
    // Add source files
    for source_file in package.source_files()? {
        let relative_path = source_file.strip_prefix(&package.root)?;
        tar.append_path_with_name(&source_file, relative_path)?;
    }
    
    tar.finish()?;
    drop(tar);
    
    Ok(archive)
}
