//! # Package Registry
//!
//! Client for interacting with the Quantum package registry.

use crate::package::Package;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use base64::Engine;

/// Default registry URL
const DEFAULT_REGISTRY_URL: &str = "https://registry.silverbitcoin.org";

/// Package registry client
pub struct Registry {
    url: String,
    client: reqwest::Client,
}

impl Registry {
    /// Create a new registry client
    pub fn new(url: Option<&str>) -> Result<Self> {
        let url = url.unwrap_or(DEFAULT_REGISTRY_URL).to_string();
        
        let client = reqwest::Client::builder()
            .user_agent(format!("quantum-cli/{}", env!("CARGO_PKG_VERSION")))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self { url, client })
    }
    
    /// Get registry URL
    pub fn url(&self) -> &str {
        &self.url
    }
    
    /// Publish a package to the registry
    pub async fn publish(&self, package: &Package, archive: Vec<u8>) -> Result<()> {
        let publish_url = format!("{}/api/v1/packages/publish", self.url);
        
        let request = PublishRequest {
            name: package.name().to_string(),
            version: package.version().to_string(),
            description: package.manifest.package.description.clone(),
            license: package.manifest.package.license.clone(),
            repository: package.manifest.package.repository.clone(),
            archive_data: base64::engine::general_purpose::STANDARD.encode(&archive),
        };
        
        let response = self.client
            .post(&publish_url)
            .json(&request)
            .send()
            .await
            .context("Failed to send publish request")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Registry error: {}", error_text);
        }
        
        Ok(())
    }

    /// Download a package from the registry
    pub async fn download(&self, name: &str, version: &str) -> Result<Vec<u8>> {
        let download_url = format!("{}/api/v1/packages/{}/{}/download", self.url, name, version);
        
        let response = self.client
            .get(&download_url)
            .send()
            .await
            .context("Failed to download package")?;
        
        if !response.status().is_success() {
            anyhow::bail!("Package not found: {} v{}", name, version);
        }
        
        let archive = response.bytes().await?.to_vec();
        
        Ok(archive)
    }
    
    /// Search for packages in the registry.
    ///
    /// Queries the registry for packages matching the search term.
    ///
    /// # Arguments
    /// * `query` - The search query string
    ///
    /// # Returns
    /// A vector of matching package information
    #[allow(dead_code)]
    pub async fn search(&self, query: &str) -> Result<Vec<PackageInfo>> {
        let search_url = format!("{}/api/v1/packages/search?q={}", self.url, query);
        
        let response = self.client
            .get(&search_url)
            .send()
            .await
            .context("Failed to search packages")?;
        
        if !response.status().is_success() {
            anyhow::bail!("Search failed");
        }
        
        let results: SearchResponse = response.json().await?;
        
        Ok(results.packages)
    }
}

/// Publish request
#[derive(Debug, Serialize)]
struct PublishRequest {
    name: String,
    version: String,
    description: Option<String>,
    license: Option<String>,
    repository: Option<String>,
    archive_data: String,
}

/// Package information from the registry.
///
/// Contains metadata about a published package:
/// - Name and version
/// - Description
/// - Download count
#[derive(Debug, Deserialize)]
pub struct PackageInfo {
    /// The package name
    #[allow(dead_code)]
    pub name: String,
    /// The package version
    #[allow(dead_code)]
    pub version: String,
    /// Optional package description
    #[allow(dead_code)]
    pub description: Option<String>,
    /// Number of downloads
    #[allow(dead_code)]
    pub downloads: u64,
}

/// Search response from the registry.
///
/// Contains the list of packages matching a search query.
#[derive(Debug, Deserialize)]
struct SearchResponse {
    /// The list of matching packages
    #[allow(dead_code)]
    packages: Vec<PackageInfo>,
}
