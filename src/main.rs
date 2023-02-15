use crate::utils::get_env;
use api::client::discord_authenticate;

mod api;
mod utils;

#[tokio::main]
async fn main() {
    let mut client = discord_authenticate(get_env("DISCORD_BOT_TOKEN").as_str()).await;
    let task = client.start();

    println!("Login succeeded");
    if let Err(why) = task.await {
        println!("An error occurred while running the client: {why:?}");
    }
}
