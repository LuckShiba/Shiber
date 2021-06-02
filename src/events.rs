use isahc::AsyncReadResponseExt;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateInteractionResponse},
    client::{Context, EventHandler},
    model::{
        channel::ReactionType,
        interactions::{
            ButtonStyle, Interaction, InteractionApplicationCommandCallbackDataFlags,
            InteractionData, InteractionMessage, InteractionResponseType,
        },
        prelude::Ready,
    },
};

use crate::{constants::EMBED_COLOR, GenericResult};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("{} ready.", &data_about_bot.user.tag());
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(why) = handle_interaction(ctx, &interaction).await {
            eprintln!("Error in interaction {:?}: {:?}", interaction.data, why);
        }
    }
}

async fn handle_interaction(ctx: Context, interaction: &Interaction) -> GenericResult<()> {
    if let Some(data) = &interaction.data {
        match data {
            InteractionData::ApplicationCommand(command) => match command.name.as_str() {
                "shiba" => {
                    let image = get_shiba_image().await?;
                    interaction
                        .create_interaction_response(&ctx, |r| create_shiba_data(r, image))
                        .await?;
                }
                _ => eprintln!("Unhandled command received: {}", command.name),
            },
            InteractionData::MessageComponent(component) => match component.custom_id.as_str() {
                "btn_shiba_reload" => {
                    if let Some(InteractionMessage::Regular(message)) = &interaction.message {
                        if let Some(old_interaction) = &message.interaction {
                            if let Some(member) = &interaction.member {
                                if old_interaction.user.id != member.user.id {
                                    interaction
                                        .create_interaction_response(&ctx, |r| {
                                            r.interaction_response_data(|d| d.
                                                content("Use `/shiba` to do that.").flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                                            )
                                        })
                                        .await?;
                                    return Ok(());
                                }
                            }
                        }
                    }
                    let image = get_shiba_image().await?;
                    let embed = create_shiba_embed(image);
                    interaction
                        .create_interaction_response(&ctx, |r| {
                            r.kind(InteractionResponseType::UpdateMessage)
                                .interaction_response_data(|d| d.set_embed(embed))
                        })
                        .await?;
                }
                _ => eprintln!("Unhandled component: {}", component.custom_id),
            },
        }
    }
    Ok(())
}

fn create_shiba_data(
    res: &mut CreateInteractionResponse,
    image: String,
) -> &mut CreateInteractionResponse {
    let embed = create_shiba_embed(image);
    res.interaction_response_data(|d| {
        d.set_embed(embed).components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| {
                    b.custom_id("btn_shiba_reload")
                        .style(ButtonStyle::Primary)
                        .label("Reload")
                        .emoji(ReactionType::Unicode("🔁".into()))
                })
            })
        })
    })
}

fn create_shiba_embed(image: String) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(EMBED_COLOR).image(image);
    embed
}

async fn get_shiba_image() -> GenericResult<String> {
    Ok(get_shiba_images(1).await?.pop().unwrap())
}

async fn get_shiba_images(count: u8) -> GenericResult<Vec<String>> {
    Ok(
        isahc::get_async(format!("https://shibe.online/api/shibes?count={}", count))
            .await?
            .json::<Vec<String>>()
            .await?,
    )
}
