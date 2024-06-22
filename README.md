# txt2json

## Overview

A Rust utility that converts tab-separated values into a JSON format. This tool reads a TSV file, extracts the headers, and then converts each line into JSON

## Features

- **Read TSV File**: Opens and reads a tab-separated values (TSV) file.
- **Extract Headers**: Extracts headers from the first line of the TSV file.
- **Convert to JSON**: Converts each subsequent line into a JSON object.
- **Output JSON File**: Writes the JSON data to `data.json`.

### Example

Given a TSV file (`src/data.txt`):

```
name	age	city
Ahmed	24	Abu Dhabi
Rashed	25	Dubai
```

The resulting JSON file (`src/data.json`) will be:

```json
{
  "data": [
    {
      "name": "Ahmed",
      "age": 24,
      "city": "Abu Dhabi"
    },
    {
      "name": "Rashed",
      "age": 25,
      "city": "Dubai"
    }
  ]
}
```

## Code Explanation

### Functions

- `init_file() -> File`: Initializes and creates the output JSON file.
- `read_table(path: &str) -> Peekable<Lines<BufReader<File>>>`: Reads the input TSV file and returns an iterator over its lines.
- `get_headers(file: &mut Peekable<Lines<BufReader<File>>>) -> Vec<String>`: Extracts headers from the TSV file.
- `get_records(file: Peekable<Lines<BufReader<File>>>, headers: Vec<String>, output: File)`: Processes each line of the TSV file, converts it into JSON format, and writes to the output file.

### Main Function

- The `main` function orchestrates the reading of the TSV file, extracting headers, converting records, and writing the final JSON output. It also measures and prints the execution time.

