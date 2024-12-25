use reqwest::blocking::get;
use reqwest::Error;
use scraper::{Html, Selector};

fn main() -> Result<(), Error> {
    // Step 1: Connect to the target page
    let response = get("https://www.scrapethissite.com/pages/simple/")?;
    
    // Step 2: Extract the raw HTML and print it
    let html = response.text()?;
  //  println!("{}", html); // Optional: Print the raw HTML
    
    // Step 3: Parse the HTML document
    let document = Html::parse_document(&html);
    
    // Step 4: Create a selector for the data you want to scrape
    let selector = Selector::parse("div.country").unwrap(); // Adjust selector based on your target data

    // Step 5: Extract data using the selector
    for element in document.select(&selector) {
        let country_name = match element.select(&Selector::parse("h3").unwrap()).next() {
            Some(el) => el.inner_html(),
            None => "Unknown Country".to_string(), // Handle the case where no element is found
        };
        
        let population = match element.select(&Selector::parse(".population").unwrap()).next() {
            Some(el) => el.inner_html(),
            None => "Unknown Population".to_string(), // Handle the case where no element is found
        };
        
        
        println!("Country: {}, Population: {}", country_name, population);
    }

    Ok(())
}
