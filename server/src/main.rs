use shared::model;

mod error;
mod fetch_data;
mod stream;

// use std::io::stdin;
// use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _handle = tokio::spawn(async {
        // let _ = stream::fetch_token().await;
        if let Err(e) = stream::fetch_token().await {
            println!("Stream error: {:}", e)
        }
        println!("new token");
    });
    loop {
        println!("1 - Get wallet balance");
        println!("2 - Get wallet transactions");
        println!("3 - Quit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                println!("Entrez une adresse");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim();
                fetch_data::get_balance(&input).await?;
            }
            "2" => {
                println!("Entrez un id de transaction");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim();
                fetch_data::get_transaction(&input).await?;
            }
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
    Ok(())
}
