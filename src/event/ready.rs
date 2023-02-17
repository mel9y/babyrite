use serenity::{async_trait, client::Context, model::prelude::Ready, prelude::EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Login succeeded");
        println!(
            "Ready! {}({}) is connected!",
            ready.user.tag(),
            ready.user.id
        )
    }
}
