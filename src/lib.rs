//! # FindAGrave Citation Parser
//!
//! This library provides functionality to fetch and parse citation information
//! from the Find a Grave website. It extracts key details such as name, birth date,
//! death date, burial location, and more.
//!
//! ## Features
//!
//! - Fetch HTML from FindAGrave memorial pages
//! - Parse and extract structured data
//! - Store data in SQLite database
//! - Error handling for web scraping operations

pub mod db;
mod error;
mod fetcher;
mod models;
mod parser;

pub use db::store_in_db;
pub use error::{Error, Result};
pub use fetcher::fetch_page;
pub use models::Memorial;
pub use parser::parse_page;

/// Configuration options for the parser
#[derive(Debug, Clone)]
pub struct Config {
    /// User agent string for HTTP requests
    pub user_agent: String,
    /// Connection timeout in seconds
    pub timeout_secs: u64,
    /// Path to SQLite database
    pub db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
            timeout_secs: 10,
            db_path: "memorials.db".to_string(),
        }
    }
}

/// Process a FindAGrave URL or memorial ID
///
/// This function takes a URL or ID, fetches the page, parses the data,
/// and optionally stores it in the database.
pub async fn process_memorial(
    url_or_id: &str,
    config: &Config,
    store_to_db: bool,
) -> Result<Memorial> {
    // Normalize input to a proper URL
    let url = if url_or_id.contains("findagrave.com") {
        url_or_id.to_string()
    } else {
        format!("https://www.findagrave.com/memorial/{}", url_or_id)
    };

    // Fetch the page
    let html = fetch_page(&url, config).await?;

    // Parse the page
    let memorial = parse_page(&html)?;

    // Store in database if requested
    if store_to_db {
        store_in_db(&memorial, &config.db_path)?;
    }

    Ok(memorial)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.timeout_secs, 10);
        assert_eq!(config.db_path, "memorials.db");
    }
}
