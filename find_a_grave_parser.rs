use reqwest;
use scraper::{Html, Selector};
use rusqlite::{params, Connection, Result};
use std::error::Error;

#[derive(Debug)]
struct Memorial {
    name: String,
    birth_date: Option<String>,
    birth_location: Option<String>,
    death_date: Option<String>,
    death_location: Option<String>,
    burial_location: Option<String>,
    plot_details: Option<String>,
}

impl Memorial {
    fn new() -> Self {
        Memorial {
            name: String::new(),
            birth_date: None,
            birth_location: None,
            death_date: None,
            death_location: None,
            burial_location: None,
            plot_details: None,
        }
    }
}

async fn fetch_page(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    let response = client.get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;
    
    Ok(response)
}

fn parse_page(html: &str) -> Result<Memorial, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let mut memorial = Memorial::new();

    // Define selectors
    let selectors = [
        (Selector::parse("[itemprop='name'], h1.name, .bio-info .full-name").unwrap(), 
         |m: &mut Memorial, v: String| m.name = v),
        (Selector::parse("[itemprop='birthDate'], .birth-info .date").unwrap(),
         |m: &mut Memorial, v: String| m.birth_date = Some(v)),
        (Selector::parse("[itemprop='birthPlace'], .birth-info .location").unwrap(),
         |m: &mut Memorial, v: String| m.birth_location = Some(v)),
        (Selector::parse("[itemprop='deathDate'], .death-info .date").unwrap(),
         |m: &mut Memorial, v: String| m.death_date = Some(v)),
        (Selector::parse("[itemprop='deathPlace'], .death-info .location").unwrap(),
         |m: &mut Memorial, v: String| m.death_location = Some(v)),
        (Selector::parse("[itemprop='burialPlace'], .burial-info .location").unwrap(),
         |m: &mut Memorial, v: String| m.burial_location = Some(v)),
        (Selector::parse(".plot-details, .grave-location").unwrap(),
         |m: &mut Memorial, v: String| m.plot_details = Some(v))
    ];

    for (selector, setter) in selectors.iter() {
        if let Some(element) = document.select(selector).next() {
            let text = element.text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            if !text.is_empty() {
                setter(&mut memorial, text);
            }
        }
    }

    Ok(memorial)
}

fn store_in_db(memorial: &Memorial, conn: &Connection) -> Result<()> {
    conn.execute_batch("
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
    ")?;

    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO memorials (
            name, birth_date, birth_location, death_date,
            death_location, burial_location, plot_details
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &memorial.name,
            &memorial.birth_date,
            &memorial.birth_location,
            &memorial.death_date,
            &memorial.death_location,
            &memorial.burial_location,
            &memorial.plot_details
        ],
    )?;

    tx.commit()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.findagrave.com/example-memorial-page";
    
    // Fetch and parse the page
    let html = fetch_page(url).await?;
    let memorial = parse_page(&html)?;
    println!("Parsed Memorial: {:?}", memorial);

    // Store in database
    let conn = Connection::open("memorials.db")?;
    store_in_db(&memorial, &conn)?;

    println!("Memorial data successfully stored in database");
    Ok(())
}
