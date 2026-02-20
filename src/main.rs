mod error;
mod fetch_data;
mod model;
mod stream;

use std::io::stdin;
use tokio::spawn;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        stream::fetch_token().await;
        println!("new token");
    });
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => println!("Get wallet balance"),
            "2" => println!("Get wallet transactions"),
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
}
