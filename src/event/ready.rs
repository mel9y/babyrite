use serenity::{async_trait, client::Context, model::prelude::Ready, prelude::EventHandler};
use tracing::log::{info, warn};

use crate::BABYRITE_VERSION;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Running babyrite v{} ....", BABYRITE_VERSION);
        info!("Login succeeded. (SessionID: {}", ready.session_id);
        info!(
            "Ready! {}({}) is connected!",
            ready.user.tag(),
            ready.user.id,
        );

        // babyrite が動作できるギルドがない場合は警告
        if ready.guilds.is_empty() {
            warn!("The connected client is not a member of any guild.")
        }
    }
}
