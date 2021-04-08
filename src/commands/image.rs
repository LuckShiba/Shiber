use reqwest::Error;
use serenity::{client::Context, framework::standard::{CommandResult, macros::{command, group}}, model::channel::Message, utils::Colour};
use crate::constants;

#[group]
#[commands(shiba, bomb)]
pub struct Image;

#[command]
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
#[aliases("shibabomb")]
async fn bomb(ctx: &Context, msg: &Message) -> CommandResult {
    let images = get_shiba_images(5).await?;
    msg.channel_id
        .send_message(ctx, |reply| reply
            .content(images.join("\n"))
            .reference_message(msg)).await?;
    Ok(())
} 

async fn get_shiba_images(count: i8) -> Result<Vec<String>, Error> {
    Ok(
        reqwest::get(&format!("https://shibe.online/api/shibes?count={}", count)[..])
            .await?
            .json::<Vec<String>>()
            .await?
    )
}