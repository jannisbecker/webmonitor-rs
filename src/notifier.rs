use futures::future;
use serde_json::{json, Value};

use crate::model::{
    DiscordNotifierOptions, EmailNotifierOptions, Job, Notifier as NotifierModel, Snapshot,
};

pub struct Notifier {}

impl Notifier {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send_notifications_for_job(
        &self,
        job: &Job,
        prev_snapshot: &Option<Snapshot>,
        new_snapshot: &Snapshot,
    ) {
        let notifiers = &job.notifiers;

        future::join_all(notifiers.into_iter().map(|notifier| async move {
            match notifier {
                NotifierModel::Discord(options) => {
                    self.send_discord_notification(job, &prev_snapshot, &new_snapshot, &options)
                        .await
                }
                NotifierModel::Email(options) => {
                    self.send_email_notification(job, &prev_snapshot, &new_snapshot, &options)
                        .await
                }
            }
        }))
        .await;
    }

    async fn send_discord_notification(
        &self,
        job: &Job,
        prev_snapshot: &Option<Snapshot>,
        new_snapshot: &Snapshot,
        options: &DiscordNotifierOptions,
    ) {
        let mut embed_fields: Vec<Value> = Vec::new();

        if let Some(snap) = prev_snapshot {
            embed_fields.push(json!({
                "name": "Previous:",
                "value": format!(
                "```html\n{}```", &snap.data)
            }))
        }

        embed_fields.push(json!({
            "name": "New:",
            "value": format!(
            "```html\n{}```", &new_snapshot.data)
        }));

        let webhook_data = json!(
            {
                "embeds": [
                    {
                        "title": format!("Job '{}' changed.", &job.name),
                        "fields": embed_fields
                    }
                ]

            }
        )
        .to_string();

        let client = reqwest::Client::new();
        let _ = client
            .post(&options.webhook_url)
            .header("Content-type", "application/json")
            .body(webhook_data)
            .send()
            .await;
    }

    async fn send_email_notification(
        &self,
        job: &Job,
        prev_snapshot: &Option<Snapshot>,
        new_snapshot: &Snapshot,
        options: &EmailNotifierOptions,
    ) {
    }
}
