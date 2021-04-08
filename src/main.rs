mod commands;
mod events;
pub mod database;
pub mod constants;

use database::create_database;
use dotenv::dotenv;
use events::Events;
use mongodb::Database;

use std::env;

use serenity::{client::Client, prelude::TypeMapKey};

use commands::configure_commands;

extern crate dotenv;

struct BotDatabase;

impl TypeMapKey for BotDatabase {
    type Value = Database;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    match create_database(&env::var("MONGO_URI").expect("mongo_uri")[..]).await {
        Ok(res) => {
            let mut client = Client::builder(
                env::var("TOKEN")
                    .expect("Invalid token."))
                .event_handler(Events)
                .framework(configure_commands())
                .await
                .expect("Error creating client.");
            {
                let mut data = client.data.write().await;
                data.insert::<BotDatabase>(res);
            }

            if let Err(why) = client.start().await {
                println!("Error durring starting client: {:?}", why);
            }
        }
        Err(why) => 
            println!("Error connecting to database: {:?}", why)   
    }
}