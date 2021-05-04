mod image;
mod help;
use serenity::{client::Context, framework::{StandardFramework, standard::{DispatchError, macros::hook}}, model::channel::Message};

pub fn configure_commands() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c
            .prefix("s.")
            .case_insensitivity(true)
        )
        .group(&image::IMAGE_GROUP)
        .help(&help::HELP_CMD)
        .on_dispatch_error(error_hook)
}

#[hook]
async fn error_hook(_: &Context, _: &Message, error: DispatchError) {
    println!("{:?}", error);
}