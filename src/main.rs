use calamine::{open_workbook_auto, Reader, Sheets};
use reqwest::header;
use scraper::{self, ElementRef, Html, Selector};
use std::collections::HashMap;
use fake_user_agent::get_chrome_rua;

fn main() {
    // Initialize headers and user-agent
    let mut headers = header::HeaderMap::new();
    let user_agent = get_chrome_rua();

    // Insert user-agent as header
    headers.insert("user-agent", user_agent.parse().unwrap());

    // Input File Path
    let product_numbers_file_path = "./input/part_numbers.xlsx";
    
    // Opening the input file
    let mut sheets: Sheets<_> = open_workbook_auto(product_numbers_file_path).unwrap();
    let binding= sheets.sheet_names();
    let sheet_name = binding.first().ok_or("No sheets found").unwrap();

    // Storing all the data in the first sheet
    let parts = sheets.worksheet_range(&sheet_name);

    // building blocking client for synchronous requests
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // Initializing different Selectors
    let a_tag = Selector::parse("a").unwrap();
    let header_tag = Selector::parse("thead").unwrap();
    let data_tag = Selector::parse("tbody").unwrap();
    let th_tag =  Selector::parse("th").unwrap();
    let td_tag =  Selector::parse("td").unwrap();
    let img_tag =  Selector::parse("img").unwrap();
    let results_tag = Selector::parse(".fade > div").unwrap();
    let product_data_tag = Selector::parse("table.tblProducts").unwrap();

    // Looping over the rows in the first sheet
    for (index,part) in parts.unwrap().rows().enumerate(){
        // Skip heading
        if index == 0{
            continue;
        }
        else if index == 15{
            break;
        }

        // get(0) because the part/product numbers are in the first column
        let part_number:String = part.get(0).unwrap().clone().to_string();

        // printing url to query
        println!("{:#?}",format!("https://www.alvordpolk.com/catalog/search?q={}",part_number));
        // Sending request to thr url
        let res = client.get(format!("https://www.alvordpolk.com/catalog/search?q={}",part_number))
        .headers(headers.clone())
        .send()
        .unwrap().text();
    
        // storing response as html doc
        let response = Html::parse_document(&res.unwrap());

        let mut result:Option<ElementRef> = None; 
        let mut url: String = String::from("");

        // Selecting all the results from that search
        let results = response.select(&results_tag);

        // Loop to determine the right product, matching the product number
        for res in results.into_iter(){
            // url of the product
            let href = res.select(&a_tag).next().unwrap().value().attr("href");
            if href.unwrap().ends_with(&part_number){
                // Match Found
                result = Some(res);
                url = href.unwrap().to_string();
                break;
            }
        }

        // Skipping the part/product number is match not found
        if result == None{
            println!("Product{} not found...",part_number);
            continue;
        }

        // Sending request to the matched product url
        let res2 = client.get("https://www.alvordpolk.com".to_owned() + &url).headers(headers.clone()).send().unwrap().text();

        let response2 = Html::parse_document(&res2.unwrap());

        // Storing the data tags to get relevent data
        let table = response2.select(&product_data_tag).next().unwrap();
        let thead = table.select(&header_tag).next().unwrap();
        let tbody = table.select(&data_tag).next().unwrap();

        // Storing all headings in Vec<String>
        let headers: Vec<String> = thead.select(&th_tag)
        .map(|element| element.text().collect::<Vec<_>>().concat().trim().to_string())
        .collect();

        // Storing all data points in Vec<String>
        let mut data: Vec<String> = tbody.select(&td_tag)
        .map(|element| element.text().collect::<Vec<_>>().concat().trim().to_string())
        .collect();

        // Getting value for first data point, as it is different from others
        data[0] = tbody.select(&td_tag).next().unwrap().select(&img_tag).next().unwrap().value().attr("alt").unwrap().to_string();
        let mut table_data: HashMap<String, String> = HashMap::new();

        // Creating a HashMap to store the data points
        for (header, value) in headers.iter().zip(data.iter()) {
                table_data.insert(header.clone(), value.clone());
            }
        
        // printing the HashMap containing the data points
        println!("{:#?}",table_data);

    }

}