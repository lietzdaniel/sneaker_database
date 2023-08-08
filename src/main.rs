mod scrape;
use rusqlite::{params, Connection, Result};
use serde_json::json;

use chrono::format::ParseError;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::open("shoes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS shoes (
        id  INTEGER PRIMARY KEY AUTOINCREMENT,
        style_id   TEXT NOT NULL,
        link TEXT NOT NULL,
        name TEXT NOT NULL,
        type TEXT NOT NULL,
        model TEXT NOT NULL,
        colorway TEXT NOT NULL,
        image TEXT NOT NULL,
        size TEXT NOT NULL,
        release_date TIMESTAMP NOT NULL,
        retail_price TEXT NOT NULL,
        last_sold_price TEXT,
        extras TEXT,
        description TEXT 
    );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS shoe_prices (
        shoe_id INTEGER NOT NULL,
        size TEXT NOT NULL,
        timestamp TIMESTAMP NOT NULL,
        price INTEGER
    );",
        [],
    )?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Starting command line interface...");
    } else {
        let functionality = &args[1];
        match functionality.as_str() {
            "add" => {
                return add_shoe().await;
            }
            "show" => {
                return show_database();
            }
            _ => {
                println!("Unknown subcommand: {}", functionality);
            }
        }
    }
    loop {
        println!("Hello, this is your shoe database manager. What would you like to do today?");
        println!("[1]   Search for a shoe to add to the database");
        println!("[2]   Make a custom entry in the database");
        println!("[3]   Remove a shoe from the database");
        println!("[4]   Show your shoe database");
        println!("[5]   Get fun facts about your collection");
        println!("[6]   Quit the database manager");

        let mut input_line = String::new();
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        match input_line.trim().parse::<u8>() {
            Ok(number) => match number {
                1 => return add_shoe().await,
                2 => todo!(),
                3 => todo!(),
                4 => return show_database(),
                5 => todo!(),
                6 => break,
                _ => continue,
            },
            Err(_) => {
                println!("Invalid input. Please try again.");
                continue;
            }
        }
    }
    Ok(())
}

pub fn show_database() -> Result<()> {
    let conn = Connection::open("shoes.db")?;
    let mut stmt = conn.prepare("SELECT * from shoes")?;
    #[derive(Debug)]
    struct Shoe {
        style_id: String,
        link: String,
        name: String,
        shoe_type: String,
        model: String,
        colorway: String,
        image: String,
        size: String,
        release_date: String,
        retail_price: String,
        last_sold_price: String,
        extras: String,
        description: String,
    }

    let shoes = stmt.query_map([], |row| {
        Ok(Shoe {
            style_id: row.get(1)?,
            link: row.get(2)?,
            name: row.get(3)?,
            shoe_type: row.get(4)?,
            model: row.get(5)?,
            colorway: row.get(6)?,
            image: row.get(7)?,
            size: row.get(8)?,
            release_date: row.get(9)?,
            retail_price: row.get(10)?,
            last_sold_price: row.get(11).unwrap_or("".to_string()),
            extras: row.get(12).unwrap_or("".to_string()),
            description: row.get(13).unwrap_or("".to_string()),
        })
    })?;
    println!("+-----------------+---------------------------------------------------------------+---------+------+");
    println!("|  Style ID       |                            Name                               |  Price  | Size |");
    println!("+-----------------+---------------------------------------------------------------+---------+------+");
    for shoe in shoes {
        match shoe {
            Ok(success_shoe) => println!(
                "| {} | {} | {} | {} |",
                fill_string_with_space(success_shoe.style_id, 15),
                fill_string_with_space(success_shoe.name, 61),
                fill_string_with_space(success_shoe.retail_price, 7),
                fill_string_with_space(success_shoe.size.to_string(), 4)
            ),
            Err(e) => {
                println!("{e}");
                continue;
            }
        }
    }
    println!("+-----------------+---------------------------------------------------------------+---------+------+");
    Ok(())
}

pub fn fill_string_with_space(mut string: String, length: usize) -> String {
    while string.len() < length {
        string.push_str(" ");
    }
    string
}

pub async fn add_shoe() -> Result<()> {
    'mainloop: loop {
        println!("/*--------------------------------------------------ADDING SHOE--------------------------------------------------*/");
        println!("Which shoe do you want to search for?");
        let mut input_line = String::new();
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        println!("Please select the model you want to add");
        let (link_vec, shoe_vec) = scrape::scrape(&input_line).await;
        let mut iterator = shoe_vec.iter().cloned();
        let mut idx = 1;
        let mut show_more = true;
        println!("[x]  Exit to main menu");
        println!("[m]  Show 5 more shoes");
        'showmore: loop {
            if show_more {
                for shoe in iterator.by_ref().take(5) {
                    println!("[{}]  {}", idx, shoe);
                    idx += 1;
                }
                show_more = false;
            }

            let mut input_int = String::new();
            std::io::stdin()
                .read_line(&mut input_int)
                .expect("Failed to read line");

            let input_str = input_int.clone();

            match input_int.trim().parse::<u8>() {
                Ok(number) => {
                    if number > link_vec.len() as u8 || number == 0 {
                        continue;
                    }
                    println!("What Size is the shoe you want to add in?");
                    let size = get_shoe_size().await;
                    let shoe_info = scrape::get_shoe_info(&link_vec[number as usize]).await;

                    // let conn = Connection::open("shoes.db")?;

                    // conn.execute("INSERT INTO shoes (style_id,link, name, type, model, colorway, image,size, release_date,retail_price,last_sold_price,extras,description) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12,?13)",
                    // params![shoe_info.get("style_id"),link_vec[number as usize],shoe_info.get("name"),shoe_info.get("type"),shoe_info.get("model"),shoe_info.get("colorway"),shoe_info.get("image"),size,convert_date(shoe_info.get("release_date")).await.unwrap(),shoe_info.get("retail_price"),shoe_info.get("price"),shoe_info.get("extras"),shoe_info.get("description")])?;
                    let client = reqwest::Client::builder().build().unwrap();

                    let shoe_data = json!({
                        "style_id": shoe_info.get("style_id"),
                        "link": link_vec[number as usize],
                        "type": shoe_info.get("type"),
                        "name": shoe_info.get("name"),

                        "model": shoe_info.get("model"),
                        "colorway": shoe_info.get("colorway"),
                        "image": shoe_info.get("image"),
                        "size": size,
                        "release_date": convert_date(shoe_info.get("release_date")).await.unwrap(),
                        "retail_price": shoe_info.get("retail_price"),
                        "last_sold_price": shoe_info.get("price"),
                        "extras": shoe_info.get("extras"),
                        "description": shoe_info.get("description")

                    });

                    let response = client
                        .post("http://127.0.0.1:8000/shoes/api")
                        .header("Content-Type", "application/json")
                        .json(&shoe_data)
                        .send()
                        .await;

                    match response {
                        Ok(_) => println!(
                            "Successfully added {} to your database!",
                            shoe_info.get("name").unwrap()
                        ),
                        Err(e) => eprintln!("Something went wrong while adding your shoe to the database. Are you sure the Server is running?, Error: {}",e)
                          
                        
                    }
                    //TODO: Captcha blocked
                    // let _ = scrape::get_prices(&link_vec[number as usize]).await; #NOTE: UNIMPLEMENTED
                    
                    break 'mainloop;
                }
                Err(_) => {
                    if input_str.len() == 2 && input_str.chars().nth(0).unwrap() == 'm' {
                        show_more = true;
                        continue 'showmore;
                    }
                    println!("Invalid input. Please try again.");
                    continue;
                }
            }
        }
    }
    Ok(())
}

async fn convert_date(date_str: Option<&String>) -> Result<String, ParseError> {
    match date_str {
        Some(date) => {
            let parsed_date = NaiveDate::parse_from_str(&(date.to_string()), "%d/%m/%Y")?;
            let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            let naive_date_time = NaiveDateTime::new(parsed_date, naive_time);
            let datetime_str = DateTime::<Utc>::from_utc(naive_date_time, Utc).to_string();
            let trimmed_datetime = datetime_str.trim_end_matches(" UTC");
            Ok(String::from(trimmed_datetime))
        }
        None => Ok("--".to_string()),
    }
}

async fn get_shoe_size() -> String {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input_size;
        if input.ends_with("\n") {
            input_size = input.trim_end_matches("\n").to_string();
        } else {
            input_size = input;
        }

        if input_size.len() > 4 {
            println!("Please enter a valid size! Too long");
            continue;
        }
        let split_on_dot = input_size.split(".").collect::<Vec<&str>>();
        let mut string_builder = String::new();
        if split_on_dot.len() > 2 {
            println!("Please enter a valid size! too much split dot");
            continue;
        } else {
            match split_on_dot[0].trim().parse::<u32>() {
                Ok(_) => {
                    if split_on_dot[0].len() <= 2 {
                        string_builder.push_str(split_on_dot[0]);
                    } else {
                        println!("{split_on_dot:?}");
                        println!("Please enter a valid size! too short of a number");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Please enter a valid size! not a number");
                    continue;
                }
            }

            if split_on_dot.len() == 2 {
                if split_on_dot[1] != "5" {
                    println!("Please enter a valid size! not 5");
                    continue;
                } else {
                    string_builder.push_str(".5");
                }
            }
            return string_builder;
        }
    }
}
