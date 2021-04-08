mod image;
mod help;

use serenity::framework::StandardFramework;

pub fn configure_commands() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c
            .prefix("s.")
            // .dynamic_prefix(async |context, message| {
            //     let data = context.data.write().await;
            //     data.insert<&str>()
            // })
            .case_insensitivity(true)
        )
        .group(&image::IMAGE_GROUP)
        .help(&help::HELP_CMD)
}