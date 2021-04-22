use crate::model::{DiscordNotifierOptions, EmailNotifierOptions, Job, Snapshot};

pub struct Notifier {}

impl Notifier {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send_notifications_for_job_change(
        &self,
        job: &Job,
        prev_snapshot: Option<Snapshot>,
        new_snapshot: Snapshot,
    ) {
    }

    async fn send_discord_notification(
        &self,
        job: &Job,
        prev_snapshot: Option<Snapshot>,
        new_snapshot: Snapshot,
        options: DiscordNotifierOptions,
    ) {
    }

    async fn send_email_notification(
        &self,
        job: &Job,
        prev_snapshot: Option<Snapshot>,
        new_snapshot: Snapshot,
        options: EmailNotifierOptions,
    ) {
    }
}
