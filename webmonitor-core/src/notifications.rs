use async_trait::async_trait;
use serde_json::{json, Value};
use similar::{ChangeTag, TextDiff};

use crate::model::{DiscordNotificationOptions, EmailNotificationOptions, Job, Snapshot};

#[async_trait]
pub trait NotificationSend {
    async fn send(&self, job: &Job, prev_snapshot: &Option<Snapshot>, new_snapshot: &Snapshot);
}

pub struct DiscordNotification {
    options: DiscordNotificationOptions,
}

impl DiscordNotification {
    pub fn from_options(options: DiscordNotificationOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl NotificationSend for DiscordNotification {
    async fn send(&self, job: &Job, prev_snapshot: &Option<Snapshot>, new_snapshot: &Snapshot) {
        let mut embed_fields: Vec<Value> = Vec::new();

        if job.show_diff {
            let diff = TextDiff::from_lines(
                match prev_snapshot {
                    Some(snap) => snap.data.as_str(),
                    None => "",
                },
                &new_snapshot.data,
            );

            let diff_content = diff
                .iter_all_changes()
                .fold(String::from(""), |mut acc, change| {
                    let sign = match change.tag() {
                        ChangeTag::Delete => "- ",
                        ChangeTag::Insert => "+ ",
                        ChangeTag::Equal => "  ",
                    };

                    acc.push_str(sign);
                    acc.push_str(&change.to_string());

                    acc
                });

            embed_fields.push(json!(
                {
                    "name": "Diff:",
                    "value": format!("```diff\n{}```", diff_content)
                }
            ));
        } else {
            if let Some(snap) = prev_snapshot {
                embed_fields.push(json!(
                    {
                        "name": "Previous:",
                        "value": format!("```html\n{}```", &snap.data)
                    }
                ));
            }
            embed_fields.push(json!(
                {
                    "name": "New:",
                    "value": format!("```html\n{}```", &new_snapshot.data)
                }
            ));
        }

        let mut request_body = json!({
            "embeds": [
                {
                    "title": format!("Job '{}' changed.", &job.name),
                    "fields": embed_fields
                }
            ]
        });

        if let Some(mentions) = &self.options.user_mentions {
            request_body["content"] = json!(mentions);
        }

        let client = reqwest::Client::new();
        let _ = client
            .post(&self.options.webhook_url)
            .header("Content-type", "application/json")
            .body(request_body.to_string())
            .send()
            .await;
    }
}

pub struct EmailNotification {
    options: EmailNotificationOptions,
}

impl EmailNotification {
    pub fn from_options(options: EmailNotificationOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl NotificationSend for EmailNotification {
    async fn send(&self, job: &Job, prev_snapshot: &Option<Snapshot>, new_snapshot: &Snapshot) {}
}
