use mongodb::{Client, Database, error::Error};
pub mod guild;

pub async fn create_database<'a>(uri: &'a str) -> Result<Database, Error>{
    Ok(Client::with_uri_str(uri).await?.database("shiber"))
}