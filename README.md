# Find a Grave Citation Parser

A Rust program that fetches and parses citation information from the Find a Grave website, extracting key details such as name, birth date, death date, burial location, and more. The data can be displayed in the console and optionally stored in a SQLite database.

## Features

- Fetches and parses data from Find a Grave memorial pages.
- Extracts key information:
- Name
- Birth date and location
- Death date and location
- Burial location
- Plot details (if available)
- Optional SQLite database storage.
- Handles errors during web scraping and parsing.

## Prerequisites

- **Rust Programming Language** Install from [the official website](https://www.rust-lang.org/).
- **SQLite** Download from [SQLite's website](https://www.sqlite.org/download.html).

- **Dependencies**
  Managed via `Cargo.toml`: - `reqwest`: HTTP requests. - `scraper`: HTML parsing. - `sqlite`: SQLite database interaction.

## Setup

### 1. Clone the Repository

git clone <https://github.com/your-username/find-a-grave-citation-parser.git>
cd find-a-grave-citation-parser``

### 2. Install Dependencies

Ensure Rust is installed, then run:

`cargo build`

### 3. Prepare Input

Create `input.txt` in the project root, listing Find a Grave memorial IDs or URLs, one per line:

`12345
67890
https://www.findagrave.com/memorial/12345/john-doe`

### 4. Configure Database (Optional)

If storing parsed data:

- Ensure SQLite is installed.
- The program creates `graves.db` automatically, or you can customize its path in the code.

### 5. Run the Program

`cargo run`

- Reads IDs from `input.txt`.
- Fetches and parses data.
- Displays extracted information.
- Optionally stores data in `graves.db`.

## How It Works

### Fetching Data

Uses `reqwest` to send GET requests and fetch memorial page HTML.

### Parsing HTML

The `scraper` crate extracts data with CSS selectors.

### Storing Data

Stores extracted details in SQLite if enabled.

### Error Logging

Logs errors and problematic IDs without halting execution.

## Example Output

    Find A Grave Memorial #12345
    Name: John Doe
    Birth: 01 Jan 1900, Springfield, IL
    Death: 15 Feb 1980, Springfield, IL
    Burial: Springfield Cemetery
    Plot: Section 1, Row 3`
    Failed IDs are logged for review.

## Code Overview

### Main Functions

- `main()`: Reads `input.txt`, fetches data, prints results, and optionally stores data.
- `find_a_grave_citation(grave_id: &str)`: Fetches and parses data for a given ID.
- `read_grave_ids_from_file(filename: &str)`: Extracts IDs from `input.txt`.

### Database Interaction (in `db` module)

- `make_grave_database()`: Initializes the database.
- `add_row_to_database()`: Inserts parsed data.
- `extract_birth()` / `extract_death()`: Extract and store respective data.

### Error Handling

Graceful error management ensures the program continues processing remaining IDs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Documentation

Full documentation for this project is available at [https://thomasvincent.github.io/rust-findagrave-citation-parser/](https://thomasvincent.github.io/rust-findagrave-citation-parser/)

To view the documentation locally:
```bash
mkdocs serve
```
