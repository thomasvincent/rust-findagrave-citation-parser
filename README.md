# FindAGrave Citation Parser

[![Crates.io](https://img.shields.io/crates/v/findagrave-citation-parser)](https://crates.io/crates/findagrave-citation-parser)
[![Build Status](https://github.com/thomasvincent/rust-findagrave-citation-parser/workflows/Rust%20CI/badge.svg)](https://github.com/thomasvincent/rust-findagrave-citation-parser/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Docs.rs](https://docs.rs/findagrave-citation-parser/badge.svg)](https://docs.rs/findagrave-citation-parser)
[![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)](https://github.com/thomasvincent/rust-findagrave-citation-parser)

A Rust library and CLI tool for fetching and parsing citation information from [Find a Grave](https://www.findagrave.com/) memorial pages.

## Features

- 🌐 Fetch memorial details from FindAGrave URLs or memorial IDs
- 📋 Parse structured data including name, birth/death dates, locations, and burial information
- 💾 Store memorials in SQLite database for offline access
- 🔍 Search stored memorials by name
- 📊 Output in text or JSON format

## Installation

### From Crates.io

```bash
cargo install findagrave-citation-parser
```

### From Source

```bash
git clone https://github.com/thomasvincent/rust-findagrave-citation-parser.git
cd rust-findagrave-citation-parser
cargo build --release
```

## Usage

### As a CLI Tool

Fetch a memorial:

```bash
findagrave-citation-parser fetch https://www.findagrave.com/memorial/123456
# Or using just the ID:
findagrave-citation-parser fetch 123456 --save
```

Search the local database:

```bash
findagrave-citation-parser search "John Smith"
```

Get a specific memorial from the database:

```bash
findagrave-citation-parser get 123456
```

Output as JSON:

```bash
findagrave-citation-parser fetch 123456 --format json
```

### As a Library

```rust
use findagrave_citation_parser::{Config, process_memorial};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use default configuration
    let config = Config::default();
    
    // Process a memorial by URL or ID
    let memorial = process_memorial("123456", &config, true).await?;
    
    // Print the formatted citation
    println!("{}", memorial.to_citation());
    
    Ok(())
}
```

## Documentation

For detailed documentation, visit:

- [API Documentation](https://thomasvincent.github.io/rust-findagrave-citation-parser/)
- [Examples](https://github.com/thomasvincent/rust-findagrave-citation-parser/tree/main/examples)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool is not affiliated with, maintained by, or in any way officially connected with Find a Grave or Ancestry.com. Use responsibly and follow Find a Grave's terms of service.