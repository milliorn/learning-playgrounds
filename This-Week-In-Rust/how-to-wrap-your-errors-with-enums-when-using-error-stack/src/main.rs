mod scraper;
use std::error::Error;

use crate::scraper::{fetch_more_info_link, fetch_steam_db, sum_numbers_from_string};

fn main() -> Result<(), Box<dyn Error>> {
    let more_info_href_link: String = fetch_more_info_link("https://example.com/")?; // Fetch the "more information" href link
                                                                                     // Print the "more information" link
    println!("More information link: {}", more_info_href_link); // --> (1)

    let (build_id, time_updated): (String, String) =
        fetch_steam_db("https://steamdb.info/app/704450/depots/?branch=public")?; // Fetch the build ID and time updated
    println!("Build ID: {}", build_id); // --> (2)
    println!("Time Updated: {}", time_updated); // --> (2)

    let number_x: u32 = "4".parse()?; // Parse the string to an unsigned 32-bit integer
    let number_y: u32 = "6".parse()?; // Parse the string to an unsigned 32-bit integer

    println!("This code is being executed!! and the code below will also be executed!! :)"); // --> (3)

    println!("Sum is: {}", number_x + number_y); // --> (3)

    let sum_numbers_string = sum_numbers_from_string("4", "6")?; // Sum the two numbers
    println!("Sum is: {}", sum_numbers_string); // --> (4)
    Ok(()) // Return an empty tuple
}
