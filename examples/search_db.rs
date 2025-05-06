use findagrave_citation_parser::db::search_memorials_by_name;
use findagrave_citation_parser::{process_memorial, Config, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the default configuration
    let config = Config::default();

    // Example memorial IDs to fetch and store in database
    let memorial_ids = ["143", "1", "1000"]; // Some example memorial IDs

    println!("Fetching and storing memorial information...");

    // Fetch and store the memorials
    for id in &memorial_ids {
        match process_memorial(id, &config, true).await {
            Ok(memorial) => {
                println!("Stored memorial for: {}", memorial.name);
            }
            Err(err) => {
                eprintln!("Error fetching memorial {}: {}", id, err);
            }
        }
    }

    // Now search the database
    println!("\nSearching database for 'Shakespeare'...");
    let results = search_memorials_by_name("Shakespeare", &config.db_path)?;

    if results.is_empty() {
        println!("No memorials found matching 'Shakespeare'");
    } else {
        println!("Found {} memorials:", results.len());
        for (i, memorial) in results.iter().enumerate() {
            println!("\n--- Memorial {} ---", i + 1);
            println!("{}", memorial.to_citation());
        }
    }

    Ok(())
}
