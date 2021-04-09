use serenity::{builder::CreateEmbed, client::Context, framework::standard::{Args, CommandResult, macros::{command, group}}, model::{channel::Message}};

use crate::{constants::EMBED_COLOR, utils::mention_or_id};

#[group]
#[summary("Commands to help to moderate your server.")]
#[only_in(guilds)]
#[commands(kick)]
pub struct Moderation;

#[command]
#[description("Kicks a member.")]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let author = msg.member(ctx).await?;
    let guild = msg.guild_id.unwrap();
    
    if author.permissions(ctx).await?.kick_members() {
        if let Some(id) = mention_or_id(msg, &mut args) {
            if let Ok(member) = guild.member(ctx, id).await {
                let reason = args
                    .remains()
                    .unwrap_or("No reason provided.");
                let success = guild
                    .kick_with_reason(
                        ctx,
                        member.user.id.0,
                        &format!("{} | {}", author.user.tag(), reason)[..]
                    )
                    .await
                    .is_ok();

                let mut embed = CreateEmbed::default();
                embed.color(EMBED_COLOR);
                if success {
                    let reason = format!("`{}`", reason);
                    embed
                        .field("Reason", reason, false)
                        .title(format!("`{}` was successfully kicked.", member.user.tag()));
                } else {
                    embed
                        .title(
                            "Cannot kick user. Please, check if I have enough permissions."
                        );
                }
                
                msg.channel_id.send_message(ctx, |m| m
                    .set_embed(embed)
                    .reference_message(msg)
                ).await?;

                return Ok(())
            }
        }
    }

    msg.channel_id.send_message(ctx, |m| m
        .embed(|embed| embed
            .color(EMBED_COLOR)
            .title("Cannot find this member.")
        )
        .reference_message(msg)
    ).await?;

    Ok(())
}