use serenity::{prelude::GatewayIntents, Client};

/// Generates a client used to login to the Discord API. If generated with an invalid token, an error will occur.
/// * `token` - token to use for the generated client
///
/// The generated client [`Client`](https://docs.rs/serenity/latest/serenity/client/struct.Client.html) is the return value.
///
/// ### Example:
/// ```
/// async fn main() {
///    let mut client = discord_client_builder(get_env("DISCORD_BOT_TOKEN").as_str()).await;
///    let task = client.start();
///
///    println!("Login succeeded");
///    if let Err(why) = task.await {
///        println!("An error occurred while running the client: {why:?}");
///    }
/// }
/// ```
pub async fn discord_client_builder(token: &str) -> Client {
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    serenity::Client::builder(token, intents)
        .await
        .expect("Failed to generate client")
}
