mod scrape;
use rusqlite::{Connection, Result};


#[tokio::main]
async fn main() ->Result<()> {
    let conn = Connection::open("shoes.db")?;
    conn.execute( "CREATE TABLE IF NOT EXISTS shoes (
        id  INTEGER PRIMARY KEY AUTOINCREMENT,
        style_id   TEXT NOT NULL,
        name TEXT NOT NULL,
        type TEXT NOT NULL,
        model TEXT NOT NULL,
        colorway TEXT NOT NULL,
        description TEXT NOT NULL,
        extras TEXT,
        retail_price FLOAT,
        release_date TIMESTAMP NOT NULL,
        image TEXT NOT NULL
    );", [])?;
    conn.execute( "CREATE TABLE IF NOT EXISTS shoe_prices (
        id  INTEGER PRIMARY KEY AUTOINCREMENT,
        style_id   TEXT NOT NULL,
        size TEXT NOT NULL,
        price TEXT,
        timestamp TIMESTAMP NOT NULL
    );", [])?;
    loop {
        println!("Hello, this is your shoe database manager. What would you like to do today?");
        println!("[1]   Search for a shoe to add to the database");
        println!("[2]   Make a custom entry in the database");
        println!("[3]   Remove a Shoe from the database");
        println!("[4]   Get Fun Facts about your collection");
        println!("[5]   Quit the database manager");

        let mut input_line = String::new();
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        match input_line.trim().parse::<u8>() {
            Ok(number) => match number {
                1 => add_shoe().await,
                2 => todo!(),
                3 => todo!(),
                4 => todo!(),
                5 => break,
                _ => continue,
            },
            Err(_) => {
                println!("Invalid input. Please try again.");
                continue; // Repeat the loop if the input is not a valid integer
            }
        }
    }
    Ok(())
}

pub async fn add_shoe() -> () {
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
        println!("[x]  Exit to Main Menu");
        println!("[m]  Show 5 more shoes");
        'showmore: loop {
            if show_more {
            for  shoe in iterator.by_ref().take(5) {
                println!("[{}]  {}", idx, shoe);
                idx += 1;
            }
            show_more = false;
        }

            let mut input_int = String::new();
            std::io::stdin()
                .read_line(&mut input_int)
                .expect("Failed to read line");
          
            let mut input_str =input_int.clone();
           
            match input_int.trim().parse::<u8>() {
                Ok(number) => {
                    if number > link_vec.len() as u8 || number == 0 {
                        continue;
                    }
                    scrape::get_shoe_info(&link_vec[number as usize]).await;

                    break 'mainloop
                }
                Err(e) => {
                 
                    if input_str.len() == 2 && input_str.chars().nth(0).unwrap() == 'm' {
                       
                        show_more = true;
                        continue 'showmore;
                    }
                    println!("Invalid input. Please try again.");
                    continue; // Repeat the loop if the input is not a valid integer
                }
            }
        }
    }
}
