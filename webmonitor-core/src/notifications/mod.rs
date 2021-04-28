use async_trait::async_trait;

use crate::model::{Job, Snapshot};

mod discord;
pub use self::discord::*;

mod email;
pub use self::email::*;

#[async_trait]
pub trait NotificationSend {
    async fn send(&self, job: &Job, prev_snapshot: &Option<Snapshot>, new_snapshot: &Snapshot);
}
