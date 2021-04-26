use futures::future;
use serde_json::{json, Value};
use similar::{ChangeTag, TextDiff};

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

        if let Some(mentions) = &options.user_mentions {
            request_body["content"] = json!(mentions);
        }

        let client = reqwest::Client::new();
        let _ = client
            .post(&options.webhook_url)
            .header("Content-type", "application/json")
            .body(request_body.to_string())
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
