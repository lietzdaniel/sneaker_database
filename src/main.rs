mod scrape;
use rusqlite::{params, Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::open("shoes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS shoes (
        id  INTEGER PRIMARY KEY AUTOINCREMENT,
        style_id   TEXT NOT NULL,
        name TEXT NOT NULL,
        type TEXT NOT NULL,
        model TEXT NOT NULL,
        colorway TEXT NOT NULL,
        image TEXT NOT NULL,
        size INTEGER NOT NULL,
        release_date TIMESTAMP NOT NULL,
        retail_price INTEGER,
        last_sold_price INTEGER,
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
    loop {
        println!("Hello, this is your shoe database manager. What would you like to do today?");
        println!("[1]   Search for a shoe to add to the database");
        println!("[2]   Make a custom entry in the database");
        println!("[3]   Remove a shoe from the database");
        println!("[4]   Get fun facts about your collection");
        println!("[5]   Quit the database manager");

        let mut input_line = String::new();
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        match input_line.trim().parse::<u8>() {
            Ok(number) => match number {
                1 => return add_shoe().await,
                2 => todo!(),
                3 => todo!(),
                4 => todo!(),
                5 => break,
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
                    println!("{shoe_info:?}");
                    let conn = Connection::open("shoes.db")?;
      
                    conn.execute("INSERT INTO shoes (style_id, name, type, model, colorway, image,size, release_date,retail_price,last_sold_price,extras,description) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                    params![shoe_info.get("style_id"),shoe_info.get("name"),shoe_info.get("type"),shoe_info.get("model"),shoe_info.get("colorway"),shoe_info.get("image"),size,shoe_info.get("release_date"),shoe_info.get("retail_price"),shoe_info.get("price"),shoe_info.get("extras"),shoe_info.get("description")])?;

                    //TODO: Captcha blocked
                    //let _ = scrape::get_prices(&link_vec[number as usize]).await; NOTE: UNIMPLEMENTED
                    println!(
                        "Successfully added {} to your database!",
                        shoe_info.get("name").unwrap()
                    );

                    break 'mainloop;
                    //TODO Add Prices
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

async fn get_shoe_size() -> String {
    loop {
        
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut input_size = String::new();
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
                }
            }
            return string_builder;
        }
    }

}
