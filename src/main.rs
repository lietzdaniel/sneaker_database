mod scrape;

#[tokio::main]
async fn main() {
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
}

pub async fn add_shoe() -> () {
    loop {
        println!("Which shoe do you want to search for?");
        let mut input_line = String::new();
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        println!("Please select the model you want to add");
        let (link_vec, shoe_vec) = scrape::scrape(&input_line).await;
        for (idx, shoe) in shoe_vec.iter().enumerate() {
            println!("[{}]  {}", idx + 1, shoe);
        }

        let mut input_int = String::new();
        std::io::stdin()
            .read_line(&mut input_int)
            .expect("Failed to read line");
        match input_int.trim().parse::<u8>() {
            Ok(number) => {
                if number > link_vec.len() as u8 || number == 0 {
                    continue;
                }
                scrape::get_shoe_info(&link_vec[number as usize]).await;
            }
            Err(_) => {
                println!("Invalid input. Please try again.");
                continue; // Repeat the loop if the input is not a valid integer
            }
        }
    }
}
