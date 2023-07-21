use core::fmt;
use error_stack::{Context, IntoReport, Report, ResultExt};
use reqwest::header::{HeaderMap, CONTENT_TYPE, REFERER, USER_AGENT};
use scraper::{Html, Selector};
use std::{error::Error, time::Duration};

#[derive(Debug)]
enum ScraperError {
    InvalidHeaderMapValue, // The value of the header map is invalid
    RequestError,          // The request failed
    SelectorError,         // The selector is invalid
} // The enum type is used to represent a value that can be one of several variants

impl fmt::Display for ScraperError {
    // Implement the Display trait for ScraperError
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the fmt method
        match self {
            // Match the ScraperError enum
            ScraperError::InvalidHeaderMapValue => {
                // If the ScraperError enum is InvalidHeaderMapValue
                write!(f, "Invalid header map value provided") // Write the error message
            }
            ScraperError::RequestError => {
                // If the ScraperError enum is RequestError
                write!(f, "Error occurred while requesting data from the webpage")
                // Write the error message
            }
            ScraperError::SelectorError => {
                // If the ScraperError enum is SelectorError
                write!(f, "An error occured while initializing new Selector") // Write the error message
            }
        }
    }
}

impl Context for ScraperError {}

// The function returns a string
pub fn fetch_more_info_link(_url: &str) -> Result<String, Box<dyn Error>> {
    // A map that holds various request headers
    let mut headers = HeaderMap::new(); // Create a new header map
    headers.insert(
        // Insert a header into the header map
        USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:105.0) Gecko/20100101 Firefox/105.0"
            .parse()
            .into_report()
            .change_context(ScraperError::InvalidHeaderMapValue)?, // --> (1)
    ); // --> (1)
    headers.insert(
        // Insert a header into the header map
        REFERER, // The header key
        "https://google.com/" // The header value
            .parse() // Parse the header value
            .into_report() // Convert the Result type to Report type
            .change_context(ScraperError::InvalidHeaderMapValue)?, // --> (1)
    );
    headers.insert(
        // Insert a header into the header map
        CONTENT_TYPE, // The header key
        "application/x-www-form-urlencoded" // The header value
            .parse() // Parse the header value
            .into_report() // Convert the Result type to Report type
            .change_context(ScraperError::InvalidHeaderMapValue)?, // --> (1)
    );

    // A blocking request call that fetches the html text from the example.com site.
    let html_results = reqwest::blocking::Client::new() // Create a new blocking reqwest client
        .get("https://example.com/") // Create a new GET request
        .timeout(Duration::from_secs(30)) // Set the timeout duration
        .headers(headers) // Set the headers
        .send() // Send the request
        .into_report() // Convert the Result type to Report type
        .change_context(ScraperError::RequestError)? // --> (2)
        .text() // Convert the response to text
        .into_report() // Convert the Result type to Report type
        .change_context(ScraperError::RequestError)?; // --> (2)

    // Parse the recieved html text to html for scraping.
    let document = Html::parse_document(&html_results); // Parse the html text to html

    // Initialize a new selector for scraping more information href link.
    let more_info_href_link_selector =
        Selector::parse("div>p>a$") // --> (3)
            .map_err(|_| Report::new(ScraperError::SelectorError)) // --> (3)
            .attach_printable_lazy(|| "invalid CSS selector provided")?; // --> (3)

    // Scrape the more information href link.
    let more_info_href_link = document // Parse the HTML text
        .select(&more_info_href_link_selector) // Select the element
        .next() // Get the first element
        .unwrap() // Unwrap the element
        .value() // Get the value of the element
        .attr("href") // Get the href attribute of the element
        .unwrap(); // Unwrap the href attribute

    // Print the more information link.
    println!("More information link: {}", more_info_href_link);

    Ok(more_info_href_link.to_owned()) // Return the "more information" href link
}

// The function returns a tuple of the build ID and time updated
pub fn fetch_steam_db(_url: &str) -> Result<(String, String), Box<dyn Error>> {
    let html = r#"<ul class="app-json"><li><i>buildid:</i> <b>11253735</b></li><li><i>timeupdated:</i> <b>1684941242</b></li></ul>"#; // HTML text to parse

    let document = Html::parse_document(html); // Parse the HTML text

    // Selector for build ID
    let build_id_selector = Selector::parse("ul.app-json li:first-child b").unwrap(); // Select the first child of the list
    let build_id = document // Parse the HTML text
        .select(&build_id_selector) // Select the element
        .next() // Get the first element
        .map(|elem| elem.text().collect::<String>()) // Get the text of the element
        .unwrap_or_else(|| "Not found".to_owned()); // If the build ID is not found, return "Not found"

    // Selector for time updated
    let time_updated_selector = Selector::parse("ul.app-json li:nth-child(2) b").unwrap(); // Select the second child of the list
    let time_updated = document // Parse the HTML text
        .select(&time_updated_selector) // Select the element
        .next() // Get the first element
        .map(|elem| elem.text().collect::<String>()) // Get the text of the element
        .unwrap_or_else(|| "Not found".to_owned()); // If the time updated is not found, return "Not found"

    Ok((build_id, time_updated)) // Return a tuple of the build ID and time updated
}

// The function returns a 32-bit unsigned integer
pub fn sum_numbers_from_string(
    number_x_as_string: &str,
    number_y_as_string: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    let number_x: u32 = number_x_as_string.parse()?; // Parse the string to an unsigned 32-bit integer
    let number_y: u32 = number_y_as_string.parse()?; // Parse the string to an unsigned 32-bit integer

    println!("This code is being executed!! and the code below will also be executed!! :)");

    Ok(number_x + number_y) // Return the sum of the two numbers
}
