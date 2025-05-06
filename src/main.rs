use clap::{Parser, Subcommand};
use findagrave_citation_parser::db::{get_memorial_by_id, search_memorials_by_name};
use findagrave_citation_parser::{process_memorial, Config, Result};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch and parse a Find a Grave memorial
    Fetch {
        /// FindAGrave URL or memorial ID
        url_or_id: String,

        /// Save results to database
        #[arg(short, long)]
        save: bool,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Search for memorials in the local database
    Search {
        /// Name to search for
        name: String,

        /// Database file path
        #[arg(short, long, default_value = "memorials.db")]
        database: PathBuf,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Get a memorial by ID from the local database
    Get {
        /// Memorial ID
        id: u64,

        /// Database file path
        #[arg(short, long, default_value = "memorials.db")]
        database: PathBuf,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Fetch {
            url_or_id,
            save,
            format,
        } => {
            let config = Config::default();

            let memorial = process_memorial(url_or_id, &config, *save).await?;

            match format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&memorial)?),
                _ => println!("{}", memorial.to_citation()),
            }
        }

        Commands::Search {
            name,
            database,
            format,
        } => {
            let memorials = search_memorials_by_name(name, database.to_str().unwrap())?;

            if memorials.is_empty() {
                println!("No memorials found matching '{}'", name);
                return Ok(());
            }

            match format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&memorials)?),
                _ => {
                    println!("Found {} memorials:", memorials.len());
                    for (i, memorial) in memorials.iter().enumerate() {
                        println!("\n--- Memorial {} ---", i + 1);
                        println!("{}", memorial.to_citation());
                    }
                }
            }
        }

        Commands::Get {
            id,
            database,
            format,
        } => {
            let memorial = get_memorial_by_id(*id, database.to_str().unwrap())?;

            match memorial {
                Some(m) => match format.as_str() {
                    "json" => println!("{}", serde_json::to_string_pretty(&m)?),
                    _ => println!("{}", m.to_citation()),
                },
                None => println!("No memorial found with ID {}", id),
            }
        }
    }

    Ok(())
}
