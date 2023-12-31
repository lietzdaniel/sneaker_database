use http::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use serde_json::json;

pub async fn scrape(shoe_name: &str) -> (Vec<String>, Vec<String>) {
    let query_shoe_name = shoe_name.replace(" ", "+");
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0").unwrap());
    headers.insert("accept", HeaderValue::from_str("application/json").unwrap());
    headers.insert(
        "accept-language",
        HeaderValue::from_str("en-US,en;q=0.9,de;q=0.8").unwrap(),
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_str("empty").unwrap());
    headers.insert("sec-fetch-mode", HeaderValue::from_str("no-cors").unwrap());
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
        Ok(resp) => {
            if resp.status().is_success() {
                handle_grid_response(&resp.text().await.unwrap()).await
            } else {
                eprintln!("Failed request with status: {}", resp.status());
                eprintln!("Response body: {}", resp.text().await.unwrap_or_default());
                (Vec::new(), Vec::new())
            }
        }
        Err(e) => {
            eprintln!("Request error: {}", e);
            (Vec::new(), Vec::new())
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
    headers.insert("User-Agent", HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0").unwrap());
    headers.insert("accept", HeaderValue::from_str("application/json").unwrap());
    headers.insert(
        "accept-language",
        HeaderValue::from_str("en-US,en;q=0.9,de;q=0.8").unwrap(),
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_str("empty").unwrap());
    headers.insert("sec-fetch-mode", HeaderValue::from_str("no-cors").unwrap());
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

pub async fn handle_shoe_response(response: &String) -> HashMap<String, String> {
    let mut shoe_info = HashMap::new();

    let document = scraper::Html::parse_document(response);
    let name_selector = scraper::Selector::parse("p.chakra-text.css-exht5z").unwrap();
    let description_selector = scraper::Selector::parse("p.chakra-text.css-16vde4f").unwrap();
    let data_selector = scraper::Selector::parse("p.chakra-text.css-wgsjnl").unwrap();
    let type_selector = scraper::Selector::parse(
        "h1.chakra-heading.css-t7k2e1[data-component=primary-product-title]",
    )
    .unwrap();
    let model_selector =
        scraper::Selector::parse("span[data-component=secondary-product-title]").unwrap();

    let image_selector = scraper::Selector::parse("img.chakra-image.css-g98gbd").unwrap();
    let price_selector = scraper::Selector::parse("p.chakra-text.css-13uklb6").unwrap();
    let description_component = document.select(&description_selector).next();
    let description_text: String;
    match description_component {
        Some(description) => description_text = description.text().collect::<String>(),
        None => description_text = String::new(),
    }

    let type_component = document.select(&type_selector).next().unwrap();
    let model_component = document.select(&model_selector).next().unwrap();
    let price_component = document.select(&price_selector).next().unwrap();
    let name_component = document.select(&name_selector).next().unwrap();
    let image_component = document.select(&image_selector).next().unwrap();
    let key_selector = scraper::Selector::parse("span.chakra-text.css-1px41cy").unwrap();
    for (key_element, content_element) in document
        .select(&key_selector)
        .zip(document.select(&data_selector))
    {
        //TODO: Convert retail_price to Int

        let mut key = key_element.text().collect::<String>();
        match key.as_str() {
            "Style" => key = "style_id".to_string(),
            "Release Date" => key = "release_date".to_string(),
            "Colourway" => key = "colorway".to_string(),
            "Retail Price" => key = "retail_price".to_string(),
            "Accessories Included" => key = "extras".to_string(),
            _ => key = key,
        }
        let value = content_element.text().collect::<String>();
        shoe_info.insert(key, value);
    }
    let price_text = price_component.text().collect::<String>();
    let name_text = name_component.text().collect::<String>();
    let mut type_text = type_component.text().collect::<String>();
    let model_text = model_component.text().collect::<String>();
    let image_link = image_component.value().attr("src").unwrap().to_string();
    type_text.truncate(type_text.len() - model_text.len());

    shoe_info.insert("description".to_string(), description_text);
    shoe_info.insert("name".to_string(), name_text);
    shoe_info.insert("type".to_string(), type_text);
    shoe_info.insert("model".to_string(), model_text);
    shoe_info.insert("image".to_string(), image_link);
    shoe_info.insert("price".to_string(), price_text);
    shoe_info
}

// pub async fn get_prices(links: &String) -> Option<String> {
    // unimplemented!("NOT WORKING DUE TO missing API");
//     println!("{}", links);
//     let query = links.split("/").collect::<Vec<&str>>()[2];

//     let post_form = json!(
//      {
//          "operationName":"GetMarketData",
//          "query":"query GetMarketData($id: String!, $currencyCode: CurrencyCode, $countryCode: String!, $marketName: String) {\n  product(id: $id) {\n    id\n    urlKey\n    market(currencyCode: $currencyCode) {\n      bidAskData(country: $countryCode, market: $marketName) {\n        highestBid\n        highestBidSize\n        lowestAsk\n        lowestAskSize\n      }\n    }\n    variants {\n      id\n      market(currencyCode: $currencyCode) {\n        bidAskData(country: $countryCode, market: $marketName) {\n          highestBid\n          highestBidSize\n          lowestAsk\n          lowestAskSize\n        }\n      }\n    }\n    ...BidButtonFragment\n    ...BidButtonContentFragment\n    ...BuySellFragment\n    ...BuySellContentFragment\n    ...XpressAskPDPFragment\n  }\n}\n\nfragment BidButtonFragment on Product {\n  id\n  title\n  urlKey\n  sizeDescriptor\n  productCategory\n  market(currencyCode: $currencyCode) {\n    bidAskData(country: $countryCode, market: $marketName) {\n      highestBid\n      highestBidSize\n      lowestAsk\n      lowestAskSize\n    }\n  }\n  media {\n    imageUrl\n  }\n  variants {\n    id\n    market(currencyCode: $currencyCode) {\n      bidAskData(country: $countryCode, market: $marketName) {\n        highestBid\n        highestBidSize\n        lowestAsk\n        lowestAskSize\n      }\n    }\n  }\n}\n\nfragment BidButtonContentFragment on Product {\n  id\n  urlKey\n  sizeDescriptor\n  productCategory\n  lockBuying\n  lockSelling\n  minimumBid(currencyCode: $currencyCode)\n  market(currencyCode: $currencyCode) {\n    bidAskData(country: $countryCode, market: $marketName) {\n      highestBid\n      highestBidSize\n      lowestAsk\n      lowestAskSize\n      numberOfAsks\n    }\n  }\n  variants {\n    id\n    market(currencyCode: $currencyCode) {\n      bidAskData(country: $countryCode, market: $marketName) {\n        highestBid\n        highestBidSize\n        lowestAsk\n        lowestAskSize\n        numberOfAsks\n      }\n    }\n  }\n}\n\nfragment BuySellFragment on Product {\n  id\n  title\n  urlKey\n  sizeDescriptor\n  productCategory\n  lockBuying\n  lockSelling\n  market(currencyCode: $currencyCode) {\n    bidAskData(country: $countryCode, market: $marketName) {\n      highestBid\n      highestBidSize\n      lowestAsk\n      lowestAskSize\n    }\n  }\n  media {\n    imageUrl\n  }\n  variants {\n    id\n    market(currencyCode: $currencyCode) {\n      bidAskData(country: $countryCode, market: $marketName) {\n        highestBid\n        highestBidSize\n        lowestAsk\n        lowestAskSize\n      }\n    }\n  }\n}\n\nfragment BuySellContentFragment on Product {\n  id\n  urlKey\n  sizeDescriptor\n  productCategory\n  lockBuying\n  lockSelling\n  market(currencyCode: $currencyCode) {\n    bidAskData(country: $countryCode, market: $marketName) {\n      highestBid\n      highestBidSize\n      lowestAsk\n      lowestAskSize\n    }\n  }\n  variants {\n    id\n    market(currencyCode: $currencyCode) {\n      bidAskData(country: $countryCode, market: $marketName) {\n        highestBid\n        highestBidSize\n        lowestAsk\n        lowestAskSize\n      }\n    }\n  }\n}\n\nfragment XpressAskPDPFragment on Product {\n  market(currencyCode: $currencyCode) {\n    state(country: $countryCode) {\n      numberOfCustodialAsks\n    }\n  }\n  variants {\n    market(currencyCode: $currencyCode) {\n      state(country: $countryCode) {\n        numberOfCustodialAsks\n      }\n    }\n  }\n})",
//          "variables":{
//              "id":query,
//              "currencyCode":"EUR",
//              "countryCode": "DE",
//              "marketName": "null"
//          }
//      }
//     );

//     let mut headers = HeaderMap::new();
//     headers.insert("User-Agent", HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0").unwrap());
//     headers.insert("accept", HeaderValue::from_str("application/json").unwrap());
//     headers.insert(
//         "accept-language",
//         HeaderValue::from_str("en-US,en;q=0.9,de;q=0.8").unwrap(),
//     );
//     headers.insert("sec-fetch-dest", HeaderValue::from_str("empty").unwrap());
//     headers.insert("sec-fetch-mode", HeaderValue::from_str("no-cors").unwrap());
//     headers.insert(
//         "sec-fetch-site",
//         HeaderValue::from_str("cross-site").unwrap(),
//     );

//     let client = reqwest::Client::builder().build().unwrap();
//     let result = client
//         .post("https://stockx.com/api/p/e")

//         .json(&post_form)
//         .headers(headers)
//         .send()
//         .await;
//     match result {
//         Ok(result_json) => {
//             println!("{}", result_json.text().await.unwrap());
//             return None;
//         }
//         Err(_) => {
//             println!("Something went wrong getting the prices.");
//             return None;
//         }
//     }
// }
