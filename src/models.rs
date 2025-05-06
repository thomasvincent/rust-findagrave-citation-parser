use serde::{Deserialize, Serialize};

/// Represents a Find a Grave memorial with parsed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memorial {
    /// FindAGrave memorial ID
    pub id: Option<u64>,
    /// Full name of the person
    pub name: String,
    /// Birth date if available
    pub birth_date: Option<String>,
    /// Birth location if available
    pub birth_location: Option<String>,
    /// Death date if available
    pub death_date: Option<String>,
    /// Death location if available
    pub death_location: Option<String>,
    /// Burial location if available
    pub burial_location: Option<String>,
    /// Plot details if available
    pub plot_details: Option<String>,
}

impl Memorial {
    /// Create a new empty Memorial
    pub fn new() -> Self {
        Memorial {
            id: None,
            name: String::new(),
            birth_date: None,
            birth_location: None,
            death_date: None,
            death_location: None,
            burial_location: None,
            plot_details: None,
        }
    }

    /// Check if the memorial has at least basic information
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && (self.birth_date.is_some()
                || self.death_date.is_some()
                || self.burial_location.is_some())
    }

    /// Create formatted citation string
    pub fn to_citation(&self) -> String {
        let mut citation = format!("Name: {}", self.name);

        if let Some(birth) = &self.birth_date {
            citation.push_str(&format!("\nBirth: {}", birth));
            if let Some(loc) = &self.birth_location {
                citation.push_str(&format!(", {}", loc));
            }
        }

        if let Some(death) = &self.death_date {
            citation.push_str(&format!("\nDeath: {}", death));
            if let Some(loc) = &self.death_location {
                citation.push_str(&format!(", {}", loc));
            }
        }

        if let Some(burial) = &self.burial_location {
            citation.push_str(&format!("\nBurial: {}", burial));
        }

        if let Some(plot) = &self.plot_details {
            citation.push_str(&format!("\nPlot: {}", plot));
        }

        citation
    }
}

impl Default for Memorial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memorial_new() {
        let memorial = Memorial::new();
        assert!(memorial.name.is_empty());
        assert!(memorial.birth_date.is_none());
    }

    #[test]
    fn test_memorial_is_valid() {
        let mut memorial = Memorial::new();
        // Empty memorial isn't valid
        assert!(!memorial.is_valid());

        // With just a name, not valid
        memorial.name = "John Doe".to_string();
        assert!(!memorial.is_valid());

        // With name and birth date, valid
        memorial.birth_date = Some("1900-01-01".to_string());
        assert!(memorial.is_valid());
    }

    #[test]
    fn test_memorial_to_citation() {
        let mut memorial = Memorial::new();
        memorial.name = "John Doe".to_string();
        memorial.birth_date = Some("January 1, 1900".to_string());
        memorial.birth_location = Some("Springfield, IL".to_string());

        let citation = memorial.to_citation();
        assert!(citation.contains("John Doe"));
        assert!(citation.contains("January 1, 1900"));
        assert!(citation.contains("Springfield, IL"));
    }
}
