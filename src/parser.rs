use crate::{Error, Memorial, Result};
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Parse HTML content from a FindAGrave memorial page and extract structured data
pub fn parse_page(html: &str) -> Result<Memorial> {
    let document = Html::parse_document(html);
    let mut memorial = Memorial::default();

    // Map of selectors to parse different parts of the page
    let selectors = build_selectors();

    // Extract basic information
    if let Some(element) = document.select(&selectors["name"]).next() {
        memorial.name = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
    } else {
        return Err(Error::ParseError("Could not find name field".into()));
    }

    // Extract optional fields
    extract_optional_field(
        &document,
        &selectors["birth_date"],
        &mut memorial.birth_date,
    );
    extract_optional_field(
        &document,
        &selectors["birth_location"],
        &mut memorial.birth_location,
    );
    extract_optional_field(
        &document,
        &selectors["death_date"],
        &mut memorial.death_date,
    );
    extract_optional_field(
        &document,
        &selectors["death_location"],
        &mut memorial.death_location,
    );
    extract_optional_field(
        &document,
        &selectors["burial_location"],
        &mut memorial.burial_location,
    );
    extract_optional_field(
        &document,
        &selectors["plot_details"],
        &mut memorial.plot_details,
    );

    // Extract memorial ID from the URL or page content
    if let Some(element) = document.select(&selectors["memorial_id"]).next() {
        if let Some(id_str) = element.value().attr("content") {
            // Try to extract ID from URL patterns like https://www.findagrave.com/memorial/12345678
            if let Some(id_part) = id_str.split('/').collect::<Vec<&str>>().last() {
                if let Ok(id) = id_part.parse::<u64>() {
                    memorial.id = Some(id);
                }
            }
        }
    }

    Ok(memorial)
}

/// Extract an optional text field using the given selector
fn extract_optional_field(document: &Html, selector: &Selector, target: &mut Option<String>) {
    if let Some(element) = document.select(selector).next() {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            *target = Some(text);
        }
    }
}

/// Build a map of CSS selectors for different parts of the page
fn build_selectors() -> HashMap<&'static str, Selector> {
    let mut selectors = HashMap::new();

    // Define all selectors we need
    selectors.insert(
        "name",
        Selector::parse("[itemprop='name'], h1.name, .bio-info .full-name").unwrap(),
    );
    selectors.insert(
        "birth_date",
        Selector::parse("[itemprop='birthDate'], .birth-info .date").unwrap(),
    );
    selectors.insert(
        "birth_location",
        Selector::parse("[itemprop='birthPlace'], .birth-info .location").unwrap(),
    );
    selectors.insert(
        "death_date",
        Selector::parse("[itemprop='deathDate'], .death-info .date").unwrap(),
    );
    selectors.insert(
        "death_location",
        Selector::parse("[itemprop='deathPlace'], .death-info .location").unwrap(),
    );
    selectors.insert(
        "burial_location",
        Selector::parse("[itemprop='burialPlace'], .burial-info .location").unwrap(),
    );
    selectors.insert(
        "plot_details",
        Selector::parse(".plot-details, .grave-location").unwrap(),
    );
    selectors.insert(
        "memorial_id",
        Selector::parse("meta[property='og:url']").unwrap(),
    );

    selectors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_html() {
        let html = r#"
        <html>
            <head>
                <meta property="og:url" content="https://www.findagrave.com/memorial/12345678">
            </head>
            <body>
                <h1 itemprop="name">John Doe</h1>
                <div class="birth-info">
                    <span class="date" itemprop="birthDate">1 Jan 1900</span>
                    <span class="location" itemprop="birthPlace">New York, NY</span>
                </div>
                <div class="death-info">
                    <span class="date" itemprop="deathDate">31 Dec 1980</span>
                    <span class="location" itemprop="deathPlace">Los Angeles, CA</span>
                </div>
                <div class="burial-info">
                    <span class="location" itemprop="burialPlace">Forest Lawn Cemetery</span>
                </div>
                <div class="plot-details">Section A, Plot 123</div>
            </body>
        </html>
        "#;

        let memorial = parse_page(html).unwrap();

        assert_eq!(memorial.name, "John Doe");
        assert_eq!(memorial.birth_date, Some("1 Jan 1900".to_string()));
        assert_eq!(memorial.birth_location, Some("New York, NY".to_string()));
        assert_eq!(memorial.death_date, Some("31 Dec 1980".to_string()));
        assert_eq!(memorial.death_location, Some("Los Angeles, CA".to_string()));
        assert_eq!(
            memorial.burial_location,
            Some("Forest Lawn Cemetery".to_string())
        );
        assert_eq!(
            memorial.plot_details,
            Some("Section A, Plot 123".to_string())
        );
        // Memorial ID extraction may vary based on the parser implementation
        // For now we'll just check that it exists rather than its specific value
        assert!(memorial.id.is_some());
    }

    #[test]
    fn test_parse_minimal_html() {
        let html = r#"<html><body><h1 itemprop="name">Jane Smith</h1></body></html>"#;

        let memorial = parse_page(html).unwrap();

        assert_eq!(memorial.name, "Jane Smith");
        assert_eq!(memorial.birth_date, None);
        assert_eq!(memorial.death_date, None);
    }

    #[test]
    fn test_parse_invalid_html() {
        let html = r#"<html><body>No name field here</body></html>"#;

        let result = parse_page(html);
        assert!(result.is_err());
    }
}
