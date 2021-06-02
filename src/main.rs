pub mod constants;
mod events;

use std::{
    env::{self, args},
    error::Error,
};

use dotenv::dotenv;
use events::Handler;
use serenity::{model::interactions::ApplicationCommand, Client};

pub type GenericError = Box<dyn Error + Send + Sync + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token =
        env::var("SHIBER_TOKEN").expect("No token provided. Set SHIBER_TOKEN env variable.");
    let application_id = env::var("SHIBER_CLIENT_ID")
        .expect("No client id provided. Set SHIBER_CLIENT_ID env variable.")
        .parse()
        .expect("Could not parse SHIBER_CLIENT_ID as u64.");
    let mut client = Client::builder(token)
        .application_id(application_id)
        .event_handler(Handler)
        .await
        .expect("Error creating client.");

    if args().nth(1) == Some("--update-commands".into()) {
        match ApplicationCommand::create_global_application_commands(
            &client.cache_and_http.http,
            |c| {
                c.create_application_command(|c| {
                    c.name("shiba").description("Sends a shiba image.")
                })
            },
        )
        .await
        {
            Ok(cmds) => println!("Created {} commands.", cmds.len()),
            Err(why) => eprintln!("Failed to create commands: {:?}", why),
        }
    }

    if let Err(why) = client.start().await {
        println!("Client errored: {:?}", why);
    }
}
