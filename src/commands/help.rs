
use serenity::{builder::CreateEmbed, client::Context, framework::standard::{Args, CommandGroup, CommandResult, HelpOptions, macros::help}, model::channel::Message};

use crate::constants::EMBED_COLOR;

#[help]
async fn help_cmd(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
    _help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
) -> CommandResult {
    let mut embed = CreateEmbed::default();
    embed.colour(EMBED_COLOR);
    if args.len() == 0 {
        embed.fields(
            groups
            .into_iter()
            .map(|group| (
                format!(
                    "{}: `{}`",
                    group.name,
                    group.options.summary
                        .unwrap_or("No description.")
                ),
                format!("`{}`",
                    group.options.commands
                        .into_iter()
                        .map(|command| command.options.names
                            .first()
                            .unwrap()
                            .to_string()
                        )
                    .collect::<Vec<String>>()
                    .join("`, `")
                ),
                false
            ))
        );
    } else {
        let cmd_name = args.single::<String>().unwrap();
        let mut cmd = None;
        'group: for group in groups.into_iter() {
            for command in group.options.commands.into_iter() {
                if command.options.names
                    .into_iter()
                    .any(|name| &name[..] == cmd_name) {
                        cmd = Some(command);
                        break 'group;
                    }
            }
        }
        if let Some(cmd) = cmd {
            if let Some((name, aliases)) = cmd.options.names.split_first() {
                embed.title(
                    format!(
                        "{}: `{}`",
                        name,
                        cmd.options.desc
                            .unwrap_or("No description.")
                    )
                );
                let mut description = String::new();
                if aliases.len() != 0 {
                    description += &format!("**Aliases:** `{}`\n", aliases.join("`, `"));
                }
                embed.description(description);
            }
        } else {
            embed.title("Command not found.");
        }
    }

    msg.channel_id.send_message(ctx, |reply| reply
        .set_embed(embed)
        .reference_message(msg)
    ).await?;
    
    Ok(())
}