import asyncio
import json
import httpx
import os
import logging
import sys
from nested_lookup import nested_lookup
from parsel import Selector

SHOES_PATH = os.path.join(".","sneaker_info_getter")
logging.basicConfig(level=logging.DEBUG, stream=sys.stdout)
logger = logging.getLogger()
# create HTTPX client with headers that resemble a web browser
client = httpx.Client(
    http2=True,
    follow_redirects=True,
    headers={
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36",
        "Accept": "text/html",
        "Accept-Encoding": "gzip",
        "Accept-Language": "en-US,en;q=0.9",
        "Cache-Control": "no-cache",
    },
)


def parse_nextjs(html: str) -> dict:
    """extract nextjs cache from page"""
    selector = Selector(html)
    data = selector.css("script#__NEXT_DATA__::text").get()
   
    if not data:
        data = selector.css("script[data-name=query]::text").get()
        data = data.split("=", 1)[-1].strip().strip(";")
    data = json.loads(data)
    return data


def scrape_product(url: str) -> dict:
    """scrape a single stockx product page for product data"""
    response = client.get(url)
    assert response.status_code == 200
    data = parse_nextjs(response.text)
    # extract all products datasets from page cache
    products = nested_lookup("product", data)
    # find the current product dataset
    try:
        product = next(p for p in products if p.get("urlKey") in str(response.url))
    except StopIteration:
        raise ValueError("Could not find product dataset in page cache", response)
    return product


def create_json(url,style_id):
    
    product_url = f"https://stockx.com{url}" 
    logger.info(f"Searching prices for {product_url} for shoe with style_id {style_id}")
    json_dict =  scrape_product(product_url)
    print(json_dict)
    json_string = json.dumps(json_dict)
    os.makedirs(os.path.join(SHOES_PATH,style_id,"prices"),exist_ok=True)
    with open(os.path.join(SHOES_PATH,style_id,"prices",f"{style_id}.json"), "w+") as json_file:
        json_file.write(json_string)


