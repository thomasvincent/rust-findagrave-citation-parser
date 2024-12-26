````markdown
# Find a Grave Citation Parser

This is a Rust program that fetches and parses citation information from the Find a Grave website. It extracts key details such as name, birth date, death date, burial location, and more. The extracted data can be displayed in the console and optionally stored in a SQLite database for later use.

## Features

- Fetches and parses data from Find a Grave memorial pages.
- Extracts key information:
  - Name
  - Birth date and location
  - Death date and location
  - Burial location
  - Plot details (if available)
- Optionally stores the extracted data in a SQLite database.
- Handles potential errors during web scraping and parsing.

## Prerequisites

Before using this program, make sure you have the following installed:

1. **Rust Programming Language**  
   You can install Rust from [the official website](https://www.rust-lang.org/).

2. **SQLite**  
   Ensure that SQLite is installed and available on your system. You can download it from [here](https://www.sqlite.org/download.html).

3. **Dependencies**  
   The program uses the following external crates:
   - `reqwest`: for making HTTP requests.
   - `scraper`: for parsing and extracting data from HTML.
   - `sqlite`: for interacting with a SQLite database.

These dependencies are managed through `Cargo.toml` (see below).

## Setup

### 1. Clone the repository

```bash
git clone https://github.com/your-username/find-a-grave-citation-parser.git
cd find-a-grave-citation-parser
```
````

### 2. Install dependencies

If you haven't already, install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/).

Then, in the project directory, run the following command to install the dependencies:

```bash
cargo build
```

This will download and compile the required libraries.

### 3. Prepare Input

Create a text file named `input.txt` in the project root directory. This file should contain a list of Find a Grave memorial IDs or URLs, one per line. For example:

```
12345
67890
https://www.findagrave.com/memorial/12345/john-doe
```

The program will automatically extract the memorial IDs from these URLs.

### 4. Database (Optional)

If you want to store the parsed data in an SQLite database:

- Ensure you have SQLite installed and accessible on your system.
- The program will automatically create a database file (`graves.db`) if it doesn't already exist. If you prefer to customize the database file path, you can modify the code.

### 5. Running the Program

Once everything is set up, you can run the program using the following command:

```bash
cargo run
```

This will:

1. Read memorial IDs from `input.txt`.
2. Fetch data from the Find a Grave website for each memorial.
3. Parse the relevant citation data (name, birth date, death date, burial location, etc.).
4. Print the extracted data to the console.
5. Optionally store the extracted data in an SQLite database (`graves.db`).

## How It Works

### 1. Fetching Data

The program fetches the memorial page by sending a GET request using `reqwest`'s blocking client. The URL is constructed using the Find a Grave memorial ID (or URL).

### 2. Parsing the HTML

The program uses the `scraper` crate to parse the HTML of the memorial page and extract the relevant details using CSS selectors.

### 3. Storing Data (Optional)

If the `CONNECT` flag is set to `true`, the extracted data is stored in a SQLite database (`graves.db`). The program has functions to insert new rows of data (name, birth, death, burial, etc.) into the database.

### 4. Error Handling

The program handles errors in fetching or parsing data and prints the relevant error messages. Failed memorial IDs are collected and reported at the end of the process.

## Code Overview

### Main Functions

#### `main()`

- Reads memorial IDs from the `input.txt` file.
- Iterates over the IDs and fetches data for each one using the `find_a_grave_citation()` function.
- Prints the extracted citation information to the console.
- Optionally stores the extracted data in a SQLite database.

#### `find_a_grave_citation(grave_id: &str)`

- Fetches the memorial page.
- Parses the HTML to extract name, birth, death, burial details.
- Optionally stores the parsed data in a SQLite database.

#### `read_grave_ids_from_file(filename: &str)`

- Reads the file `input.txt` to extract memorial IDs or URLs.
- Extracts the memorial ID from URLs if provided.

### Database Interaction (in the `db` module)

The `db` module contains functions for interacting with the SQLite database. This includes:

- `make_grave_database()`: Initializes the database (creates tables).
- `add_row_to_database()`: Inserts a row into the database.
- `extract_birth()`: Extracts and stores birth data.
- `extract_death()`: Extracts and stores death data.

### Error Handling

If an error occurs during the web fetching or parsing process, the error is captured and printed to the console. The program continues processing the remaining memorial IDs.

## Example Output

For each successfully parsed memorial, the following information will be displayed in the console:

```
Find A Grave Memorial #12345
Name: John Doe
Birth: 01 Jan 1900, Springfield, IL
Death: 15 Feb 1980, Springfield, IL
Burial: Springfield Cemetery
Plot: Section 1, Row 3
```

Failed or problematic IDs will be recorded and reported at the end of the process.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```

### Additional Notes

1. **Input Handling**: The program can handle both raw Find a Grave IDs (e.g., `12345`) or full URLs (e.g., `https://www.findagrave.com/memorial/12345/john-doe`). It will automatically extract the ID from the URL.

2. **Database**: The database schema is simple, but you can extend the `db` module to handle more fields or data validation.

3. **Customizations**: You can modify the program to handle different formats, more information, or even scrape additional fields if needed.

4. **Error Reporting**: The program logs problematic grave IDs for manual review or further investigation.

```
