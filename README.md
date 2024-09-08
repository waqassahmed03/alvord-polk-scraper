# Alvord-Polk Scraper

A Rust application that scrapes product data from the Alvord Polk website using product numbers stored in an Excel file.

## Features

- Reads product numbers from an Excel file.
- Sends HTTP requests to search for products on the website.
- Matches product numbers and retrieves detailed product information.
- Extracts product data and stores it in a `HashMap`.
- Utilizes fake user agents to avoid detection while scraping.

## Dependencies

This project uses the following Rust crates:

- [`calamine`](https://crates.io/crates/calamine) - For reading Excel files.
- [`reqwest`](https://crates.io/crates/reqwest) - For making HTTP requests.
- [`scraper`](https://crates.io/crates/scraper) - For parsing HTML and extracting data.
- [`fake_user_agent`](https://crates.io/crates/fake-user-agent) - For generating fake user-agent strings.

## Installation

1. **Clone the repository:**

   ```sh
   git clone https://github.com/waqassahmed03/alvord-polk-scraper.git
   cd alvord-polk-scraper
   ```

2. **Build the project:**

   ```sh
   cargo build --release
   ```

3. **Run the project:**

   ```sh
   cargo run
   ```

## Usage

1. Place your Excel file containing product numbers in the `input` folder with the name `product_numbers.xlsx`.
2. Run the application to start scraping.
3. The output will be displayed in the console, including the matched product details.

## File Structure

```
.
├── LICENSE
├── Cargo.lock    
├── Cargo.toml                  # Project dependencies and metadata
├── src
│   └── main.rs                 # Main Rust source code
└── input
    └── product_numbers.xlsx    # Excel file containing product numbers
```

## Example Output

The application will print the product data in the following format:

```
{
    "Size": "4.0MM",
    "Decimal Equiv.(in.)": "0.1575",
    "Length Overall(in.)": "4",
    "EDP Number": "01069",
    "Length of Flute(in.)": "1",
    "Price": "$32.05",
    "Diameter of Shank": ".1510-.1500",
    "description": " High Speed Steel, Straight Shank, 127-1 Right Hand Spiral, Right Hand Cut ",
    "In Stock": "Available for Shipment",
}
```

## Contributing

Feel free to fork this repository, open issues, or submit pull requests.

You can contact me at waqassahmed03@gmail.com

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

