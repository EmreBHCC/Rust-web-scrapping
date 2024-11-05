use scraper;
use reqwest;
use csv;
use std::error::Error;

struct Product {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut products: Vec<Product> = Vec::new();

    for page_number in 1..=12 {
        // Construct the URL for each page
        let url = format!("https://www.scrapingcourse.com/ecommerce/page/{}/", page_number);

        // Download the target HTML document
        let response = reqwest::blocking::get(&url)?;

        // If the page doesn't exist or cannot be accessed, skip to the next page
        if response.status().is_client_error() || response.status().is_server_error() {
            eprintln!("Page {} could not be accessed.", page_number);
            continue;
        }

        // Get the HTML content from the request response
        let html_content = response.text()?;
        let document = scraper::Html::parse_document(&html_content);

        // Define the CSS selector to get all products on the page
        let html_product_selector = scraper::Selector::parse("li.product").unwrap();
        let html_products = document.select(&html_product_selector);

        // Iterate over each HTML product to extract data
        for html_product in html_products {
            let url = html_product
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned);
            let image = html_product
                .select(&scraper::Selector::parse("img").unwrap())
                .next()
                .and_then(|img| img.value().attr("src"))
                .map(str::to_owned);
            let name = html_product
                .select(&scraper::Selector::parse("h2").unwrap())
                .next()
                .map(|h2| h2.text().collect::<String>());
            let price = html_product
                .select(&scraper::Selector::parse(".price").unwrap())
                .next()
                .map(|price| price.text().collect::<String>());

            let product = Product {
                url,
                image,
                name,
                price,
            };

            products.push(product);
        }
    }

    // Write data to CSV
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path)?;

    // Append the header to the CSV
    writer.write_record(&["url", "image", "name", "price"])?;

    // Populate the output file
    for product in products {
        writer.write_record(&[
            product.url.unwrap_or_default(),
            product.image.unwrap_or_default(),
            product.name.unwrap_or_default(),
            product.price.unwrap_or_default(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}