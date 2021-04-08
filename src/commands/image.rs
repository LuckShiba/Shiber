use std::error::Error;

use serenity::{client::Context, framework::standard::{CommandResult, macros::{command, group}}, model::channel::Message, utils::Colour};
use isahc::prelude::AsyncReadResponseExt;
use crate::constants;

#[group]
#[summary("Commands with images.")]
#[commands(shiba, bomb)]
pub struct Image;

#[command]
#[description("A shiba picture.")]
async fn shiba(ctx: &Context, msg: &Message) -> CommandResult {
    let images = get_shiba_images(1).await?;
    msg.channel_id
        .send_message(ctx, |reply| reply
            .embed(|embed| embed
                .image(&images[0])
                .color(Colour::new(constants::EMBED_COLOR)))
            .reference_message(msg)).await?;
    Ok(())
}

#[command]
#[description("Five shiba pictures.")]
#[aliases("shibabomb")]
async fn bomb(ctx: &Context, msg: &Message) -> CommandResult {
    let images = get_shiba_images(5).await?;
    msg.channel_id
        .send_message(ctx, |reply| reply
            .content(images.join("\n"))
            .reference_message(msg)).await?;
    Ok(())
} 

async fn get_shiba_images(count: i8) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    Ok(
        isahc::get_async(&format!("https://shibe.online/api/shibes?count={}", count)[..])
            .await?
            .json::<Vec<String>>()
            .await?
    )
}