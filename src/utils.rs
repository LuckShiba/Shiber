use serenity::{framework::standard::Args, model::channel::Message};

pub fn mention_or_id(msg: &Message, args: &mut Args) -> Option<u64> {
    if let Some(mention) = msg.mentions.first() {
        args.advance();
        Some(mention.id.0)
    }
    else if let Ok(id) = args.single::<u64>() {
        Some(id)
    }
    else {
        None
    }
}