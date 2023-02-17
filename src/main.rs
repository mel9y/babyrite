use crate::utils::get_env;
use api::client::discord_client_builder;

mod api;
mod event;
mod utils;

#[tokio::main]
async fn main() {
    let mut client = discord_client_builder(get_env("DISCORD_BOT_TOKEN").as_str()).await;
    let task = client.start();

    if let Err(why) = task.await {
        println!("An error occurred while running the client: {why:?}");
    }
}
