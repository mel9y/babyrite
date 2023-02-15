use serenity::{prelude::GatewayIntents, Client};

pub async fn discord_authenticate(token: &str) -> Client {
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    serenity::Client::builder(token, intents)
        .await
        .expect("Failed to generate client")
}
