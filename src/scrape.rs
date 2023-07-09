use http::{HeaderMap, HeaderValue};
use std::{collections::HashMap, vec};
pub async fn scrape(shoe_name: &str) -> (Vec<String>, Vec<String>) {
    let query_shoe_name = shoe_name.replace(" ", "+");

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36").unwrap());
    headers.insert("accept", HeaderValue::from_str("application/json").unwrap());
    headers.insert(
        "accept-language",
        HeaderValue::from_str("en-US,en;q=0.9").unwrap(),
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_str("empty").unwrap());
    headers.insert("sec-fetch-mode", HeaderValue::from_str("'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36',").unwrap());
    headers.insert(
        "sec-fetch-site",
        HeaderValue::from_str("cross-site").unwrap(),
    );

    let client = reqwest::Client::builder().build().unwrap();
    let response = client
        .get(format!(
            "https://stockx.com/en-gb/search?s={}",
            query_shoe_name
        ))
        .headers(headers)
        .send()
        .await;

    match response {
        Ok(s) => return handle_grid_response(&s.text().await.unwrap()).await,
        Err(e) => {
            println!("{e}");
            return ([].to_vec(),[].to_vec());
        }
    }
}

pub async fn handle_grid_response(response: &String) -> (Vec<String>, Vec<String>) {
    let mut shoe_vec: Vec<String> = Vec::new();
    let mut link_vec: Vec<String> = Vec::new();
    let document = scraper::Html::parse_document(response);
    let selector = scraper::Selector::parse("a[data-testid=RouterSwitcherLink]").unwrap();

    for element in document.select(&selector).take(5) {
        if let Some(href) = element.value().attr("href") {
            
            shoe_vec.push(href.to_string());
        }
    }
    let selector = scraper::Selector::parse("p.chakra-text.css-3lpefb").unwrap();
    for element in document.select(&selector).take(5) {
        println!("{}", element.text().collect::<String>());
        link_vec.push(element.text().collect::<String>());
    }
    (shoe_vec, link_vec)
}

pub async fn get_shoe_info(links: &String) -> Vec<String>{
    let mut query_shoe_url: String = String::from("https://stockx.com");
   
    query_shoe_url.push_str(links);
    println!("{}",query_shoe_url);
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36").unwrap());
    headers.insert("accept", HeaderValue::from_str("application/json").unwrap());
    headers.insert(
        "accept-language",
        HeaderValue::from_str("en-US,en;q=0.9").unwrap(),
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_str("empty").unwrap());
    headers.insert("sec-fetch-mode", HeaderValue::from_str("'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36',").unwrap());
    headers.insert(
        "sec-fetch-site",
        HeaderValue::from_str("cross-site").unwrap(),
    );

    let client = reqwest::Client::builder().build().unwrap();
    let response = client
        .get(query_shoe_url)
        .headers(headers)
        .send()
        .await;
    match response {
        Ok(s) => return handle_shoe_response(&s.text().await.unwrap()).await,
        Err(e) => {
            println!("{e}");
            return ([].to_vec());
        }
    }
    
}


pub async fn handle_shoe_response(response: &String) -> (Vec<String>) {
   
    let mut info_vec: Vec<String> = Vec::new();

    let document = scraper::Html::parse_document(response);

    let primary_selector = scraper::Selector::parse("h1.chakra-heading.css-t7k2e1[data-component=primary-product-title]").unwrap();
    let secondary_selector = scraper::Selector::parse("span[data-component=secondary-product-title]").unwrap();

    let primary_component = document.select(&primary_selector).next().unwrap();
    let secondary_component = document.select(&secondary_selector).next().unwrap();

    let primary_text = primary_component.text().collect::<String>();
    let secondary_text = secondary_component.text().collect::<String>();

    println!("Primary Product Component: {}", primary_text);
    println!("Secondary Product Component: {}", secondary_text);
   
    [].to_vec()
}
