use crate::model::{EmailNotificationOptions, Job, Snapshot};
use async_trait::async_trait;

use super::NotificationSend;

pub struct EmailNotification {
    options: EmailNotificationOptions,
}

impl EmailNotification {
    pub fn with_options(options: EmailNotificationOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl NotificationSend for EmailNotification {
    async fn send(&self, job: &Job, prev_snapshot: &Option<Snapshot>, new_snapshot: &Snapshot) {}
}
