use serenity::{client::Context, framework::standard::{Args, CommandResult, macros::{command, group}}, model::{channel::Message}};

use crate::utils::mention_or_id;

#[group]
#[summary("Commands to help to moderate your server.")]
#[only_in(guilds)]
#[commands(kick)]
pub struct Moderation;

#[command]
#[description("Kicks a member.")]
async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let author = msg.member(ctx).await?;
    let guild = msg.guild_id.unwrap();
    
    if author.permissions(ctx).await?.kick_members() {
        if let Some(id) = mention_or_id(msg, args) {
            if let Ok(member) = guild.member(ctx, id).await {
                msg.channel_id.say(ctx, member.user.tag()).await?;
            }
        }
    }

    Ok(())
}