use http::{HeaderMap, HeaderValue};
use std::{collections::HashMap, vec, hash::Hash};


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
            return ([].to_vec(), [].to_vec());
        }
    }
}

pub async fn handle_grid_response(response: &String) -> (Vec<String>, Vec<String>) {
    let mut shoe_vec: Vec<String> = Vec::new();
    let mut link_vec: Vec<String> = Vec::new();
    let document = scraper::Html::parse_document(response);
    let selector = scraper::Selector::parse("a[data-testid=RouterSwitcherLink]").unwrap();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            shoe_vec.push(href.to_string());
        }
    }
    let selector = scraper::Selector::parse("p.chakra-text.css-3lpefb").unwrap();
    for element in document.select(&selector) {
       
        link_vec.push(element.text().collect::<String>());
    }
    (shoe_vec, link_vec)
}

pub async fn get_shoe_info(links: &String) -> HashMap<String, String> {
    let mut query_shoe_url: String = String::from("https://stockx.com");

    query_shoe_url.push_str(links);
   
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
    let response = client.get(query_shoe_url).headers(headers).send().await;
    match response {
        Ok(s) => return handle_shoe_response(&s.text().await.unwrap()).await,
        Err(e) => {
            println!("{e}");
            return HashMap::new();
        }
    }
}

pub async fn handle_shoe_response(response: &String) -> HashMap<String,String> {
    let mut info_vec: Vec<String> = Vec::new();
    let mut shoe_info = HashMap::new();

    let document = scraper::Html::parse_document(response);
    let name_selector = scraper::Selector::parse("p.chakra-text.css-exht5z",).unwrap();
    let description_selector = scraper::Selector::parse("p.chakra-text.css-16vde4f",).unwrap();
    let data_selector = scraper::Selector::parse("p.chakra-text.css-wgsjnl",).unwrap();
    let type_selector = scraper::Selector::parse(  "h1.chakra-heading.css-t7k2e1[data-component=primary-product-title]",).unwrap();
    let model_selector =scraper::Selector::parse("span[data-component=secondary-product-title]").unwrap();
   
    let image_selector = scraper::Selector::parse("img.chakra-image.css-g98gbd").unwrap();
    
    let description_component = document.select(&description_selector).next().unwrap();
    let type_component = document.select(&type_selector).next().unwrap();
    let model_component = document.select(&model_selector).next().unwrap();
    
    let name_component = document.select(&name_selector).next().unwrap();
    let image_component =  document.select(&image_selector).next().unwrap(); 
    let keys = ["style-id","colorway","retail_price","release_date","extras"];
    for (idx,element) in  document.select(&data_selector).enumerate() {
        if idx >= keys.len() {
            shoe_info.insert("extras".to_string(),"".to_string()); //Extras is optional, so it may not be found
            break;
        }
            shoe_info.insert(keys[idx].to_string(), element.text().collect::<String>());
        
    }

    let description_text = description_component.text().collect::<String>();
    let name_text = name_component.text().collect::<String>();
    let mut type_text = type_component.text().collect::<String>();
    let model_text = model_component.text().collect::<String>();
    let image_link = image_component.value().attr("src").unwrap().to_string();
    type_text.truncate(type_text.len()-model_text.len());
   
    shoe_info.insert("description".to_string(), description_text);
    shoe_info.insert("name".to_string(), name_text);
    shoe_info.insert("type".to_string(), type_text);
    shoe_info.insert("model".to_string(), model_text);
    shoe_info.insert("image".to_string(), image_link);
    println!("{shoe_info:?}");
   
    shoe_info
}
