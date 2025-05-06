use findagrave_citation_parser::{process_memorial, Config, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the default configuration
    let config = Config::default();

    // Example memorial ID (a public one)
    let memorial_id = "143;"; // This is the memorial ID for William Shakespeare

    println!("Fetching memorial information for ID: {}", memorial_id);

    // Process the memorial without saving to database
    match process_memorial(memorial_id, &config, false).await {
        Ok(memorial) => {
            println!("\nSuccessfully fetched memorial information:");
            println!("{}", memorial.to_citation());

            println!("\nRaw memorial data:");
            println!("{:#?}", memorial);
        }
        Err(err) => {
            eprintln!("Error fetching memorial: {}", err);
        }
    }

    Ok(())
}
