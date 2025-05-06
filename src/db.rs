use crate::{Error, Memorial, Result};
use rusqlite::{params, Connection};
use std::path::Path;

/// Store a memorial in the SQLite database
pub fn store_in_db(memorial: &Memorial, db_path: &str) -> Result<()> {
    // Open or create the database file
    let mut conn = Connection::open(db_path).map_err(|e| Error::DatabaseError(e.to_string()))?;

    // Initialize the database schema
    initialize_db(&conn)?;

    // Start a transaction for inserting the data
    let tx = conn
        .transaction()
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    // Insert the memorial data
    tx.execute(
        "INSERT INTO memorials (
            id, name, birth_date, birth_location, death_date,
            death_location, burial_location, plot_details
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8
        ) ON CONFLICT(id) DO UPDATE SET
            name = ?2,
            birth_date = ?3,
            birth_location = ?4,
            death_date = ?5,
            death_location = ?6,
            burial_location = ?7,
            plot_details = ?8,
            updated_at = CURRENT_TIMESTAMP
        ",
        params![
            &memorial.id,
            &memorial.name,
            &memorial.birth_date,
            &memorial.birth_location,
            &memorial.death_date,
            &memorial.death_location,
            &memorial.burial_location,
            &memorial.plot_details
        ],
    )
    .map_err(|e| Error::DatabaseError(e.to_string()))?;

    // Commit the transaction
    tx.commit()
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    Ok(())
}

/// Retrieve a memorial from the database by ID
pub fn get_memorial_by_id(id: u64, db_path: &str) -> Result<Option<Memorial>> {
    if !Path::new(db_path).exists() {
        return Ok(None);
    }

    let conn = Connection::open(db_path).map_err(|e| Error::DatabaseError(e.to_string()))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, name, birth_date, birth_location, death_date, 
         death_location, burial_location, plot_details
         FROM memorials WHERE id = ?1",
        )
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    let memorial_result = stmt.query_row(params![id], |row| {
        Ok(Memorial {
            id: row.get(0).ok(),
            name: row.get(1)?,
            birth_date: row.get(2).ok(),
            birth_location: row.get(3).ok(),
            death_date: row.get(4).ok(),
            death_location: row.get(5).ok(),
            burial_location: row.get(6).ok(),
            plot_details: row.get(7).ok(),
        })
    });

    match memorial_result {
        Ok(memorial) => Ok(Some(memorial)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(Error::DatabaseError(e.to_string())),
    }
}

/// Search memorials in the database by name
pub fn search_memorials_by_name(name: &str, db_path: &str) -> Result<Vec<Memorial>> {
    if !Path::new(db_path).exists() {
        return Ok(Vec::new());
    }

    let conn = Connection::open(db_path).map_err(|e| Error::DatabaseError(e.to_string()))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, name, birth_date, birth_location, death_date,
         death_location, burial_location, plot_details
         FROM memorials WHERE name LIKE ?1
         ORDER BY name LIMIT 100",
        )
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    let search_pattern = format!("%{}%", name);
    let memorial_iter = stmt
        .query_map(params![search_pattern], |row| {
            Ok(Memorial {
                id: row.get(0).ok(),
                name: row.get(1)?,
                birth_date: row.get(2).ok(),
                birth_location: row.get(3).ok(),
                death_date: row.get(4).ok(),
                death_location: row.get(5).ok(),
                burial_location: row.get(6).ok(),
                plot_details: row.get(7).ok(),
            })
        })
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    let mut results = Vec::new();
    for memorial in memorial_iter {
        results.push(memorial.map_err(|e| Error::DatabaseError(e.to_string()))?);
    }

    Ok(results)
}

/// Initialize the database schema if it doesn't exist yet
fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        
        CREATE TABLE IF NOT EXISTS memorials (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            birth_date TEXT,
            birth_location TEXT,
            death_date TEXT,
            death_location TEXT,
            burial_location TEXT,
            plot_details TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE INDEX IF NOT EXISTS idx_memorial_name ON memorials(name);
        CREATE INDEX IF NOT EXISTS idx_memorial_dates ON memorials(birth_date, death_date);
    ",
    )
    .map_err(|e| Error::DatabaseError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_memorial() -> Memorial {
        Memorial {
            id: Some(12345),
            name: "Test Person".to_string(),
            birth_date: Some("1900-01-01".to_string()),
            birth_location: Some("Test City".to_string()),
            death_date: Some("1980-12-31".to_string()),
            death_location: Some("Another City".to_string()),
            burial_location: Some("Test Cemetery".to_string()),
            plot_details: Some("Section X, Plot 123".to_string()),
        }
    }

    #[test]
    fn test_store_and_retrieve() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db_path_str = db_path.to_str().unwrap();

        let memorial = create_test_memorial();

        // Store the memorial
        store_in_db(&memorial, db_path_str).unwrap();

        // Retrieve it
        let retrieved = get_memorial_by_id(12345, db_path_str).unwrap().unwrap();

        assert_eq!(retrieved.id, memorial.id);
        assert_eq!(retrieved.name, memorial.name);
        assert_eq!(retrieved.birth_date, memorial.birth_date);

        // Clean up
        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_search_by_name() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_search.db");
        let db_path_str = db_path.to_str().unwrap();

        // Create and store a memorial
        let memorial = create_test_memorial();
        store_in_db(&memorial, db_path_str).unwrap();

        // Search for it
        let results = search_memorials_by_name("Test", db_path_str).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Test Person");

        // Search for non-existent record
        let no_results = search_memorials_by_name("Nonexistent", db_path_str).unwrap();
        assert_eq!(no_results.len(), 0);

        // Clean up
        fs::remove_file(db_path).unwrap();
    }
}
