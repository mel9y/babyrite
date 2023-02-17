use serenity::{async_trait, client::Context, model::prelude::Ready, prelude::EventHandler};

use crate::BABYRITE_VERSION;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Login succeeded");
        println!("Running babyrite v{} ....", BABYRITE_VERSION);
        println!(
            "Ready! {}({}) is connected!",
            ready.user.tag(),
            ready.user.id
        )
    }
}
