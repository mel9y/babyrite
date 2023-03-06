use anyhow::{Ok, Result};
use regex::Regex;
use serenity::{model::channel::Message, prelude::Context};

pub async fn handle(_: &Context, msg: Message) -> Result<()> {
    let link_regix =
        Regex::new(r"https://(?:ptb\.|canary\.)?discord(?:app)?\.com/channels/(\d+)/(\d+)/(\d+)")
            .unwrap();
    let skip_regix = Regex::new(r"<.*?>").unwrap();

    if msg.author.bot {
        return Ok(());
    }

    if msg.is_private() {
        return Ok(());
    }

    let content = skip_regix.replace_all(msg.content.as_str(), "");

    if !link_regix.is_match(content.into_owned().as_str()) {
        return Ok(());
    }

    Ok(())
}
