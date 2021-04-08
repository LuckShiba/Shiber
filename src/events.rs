use serenity::{async_trait, client::{Context, EventHandler}, model::{channel::Message, prelude::Ready}};

use crate::constants::EMBED_COLOR;

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn ready(&self, _ctx: Context, data: Ready) {
        println!("{} online!", data.user.tag());
    }

    async fn message(&self, ctx: Context, message: Message) {
        if let Ok(user) = ctx.http.get_current_user().await {
            let format_mention = |chr: &str| -> String {
                format!("<@{}{}>", chr, user.id)
            };

            if message.content == format_mention("") || message.content == format_mention("!") {
                let _ = message.channel_id.send_message(ctx, |reply| reply
                .embed(|embed| embed
                    .colour(EMBED_COLOR)
                    .title(format!("Hi! My prefix is {prefix}! Use `{prefix}help` to see them.", prefix="s.")))).await;
            }
        }
             
    }
}
