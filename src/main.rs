use scraper;
use reqwest;
use csv;

// define a custom data structure
// to store the scraped data
struct Product {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

fn main() {
    // initialize the vector that will store the scraped data
    let mut products: Vec<Product> = Vec::new();
    // download the target HTML document
    let response = reqwest::blocking::get("https://www.scrapingcourse.com/ecommerce/");
    // get the HTML content from the request response
    let html_content = response.unwrap().text().unwrap();
    // parse the HTML document
    let document = scraper::Html::parse_document(&html_content);

    // define the CSS selector to get all product
    // on the page
    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    // apply the CSS selector to get all products
    let html_products = document.select(&html_product_selector);

    // iterate over each HTML product to extract data
    // from it
    for html_product in html_products {
        // scraping logic to retrieve the info
        // of interest
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

        // instanciate a new product
        // with the scraped data and add it to the list
        let product = Product {
            url,
            image,
            name,
            price,
        };

        products.push(product);
    }
    // create the CSV output file
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // append the header to the CSV
    writer
        .write_record(&["url", "image", "name", "price"])
        .unwrap();

    // populate the output file
    for product in products {
        let url = product.url.unwrap();
        let image = product.image.unwrap();
        let name = product.name.unwrap();
        let price = product.price.unwrap();
        writer.write_record(&[url, image, name, price]).unwrap();
    }

    // free up the writer resources
    writer.flush().unwrap();
}

