mod commands;
mod events;
pub mod constants;
pub mod utils;

use dotenv::dotenv;
use events::Events;

use std::env;

use serenity::client::Client;

use commands::configure_commands;

extern crate dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();            
    let mut client = Client::builder(
        env::var("TOKEN")
            .expect("Invalid token."))
        .event_handler(Events)
        .framework(configure_commands())
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        println!("Error durring starting client: {:?}", why);
}
}