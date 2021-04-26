use std::sync::Arc;

use futures::future;
use scraper::{Html, Selector};

use crate::{
    error::{FilterError, WatcherError},
    filters::{CSSFilter, FilterApply, Html2TextFilter, XPathFilter},
    model::{CSSFilterOptions, Filter, InsertableSnapshot, Job, Notification},
    notifications::{DiscordNotification, EmailNotification, NotificationSend},
    repository::Repository,
};

pub struct WebsiteMonitor {
    db: Arc<Repository>,
}

impl WebsiteMonitor {
    pub fn new(db: Arc<Repository>) -> Self {
        Self { db }
    }

    pub async fn run_website_check_for_job(&self, job: &Job) -> Result<(), WatcherError> {
        let website_dom = reqwest::get(&job.url).await?.text().await?;

        let filtered_dom = self.apply_filters(website_dom, &job.filters)?;

        let prev_snapshot = self.db.snapshots_get_latest(&job.id).await?;

        if prev_snapshot.is_none()
            || self.dom_has_changed(&prev_snapshot.clone().unwrap().data, &filtered_dom)
        {
            let data = InsertableSnapshot {
                job_id: (&job.id).clone(),
                data: filtered_dom,
            };
            let new_snapshot = self.db.snapshots_add(data).await?;

            let notifications = &job.notifications;
            let new_snap = &new_snapshot;
            let prev_snap = &prev_snapshot;

            future::join_all(notifications.into_iter().map(|notification| async move {
                match notification {
                    Notification::Discord(options) => {
                        DiscordNotification::with_options(options.clone())
                            .send(job, &prev_snap, &new_snap)
                            .await
                    }
                    Notification::Email(options) => {
                        EmailNotification::with_options(options.clone())
                            .send(job, &prev_snap, &new_snap)
                            .await
                    }
                };
            }))
            .await;
        }

        Ok(())
    }

    fn apply_filters(&self, dom: String, filters: &Vec<Filter>) -> Result<String, FilterError> {
        filters
            .into_iter()
            .try_fold(dom, |filtered_dom, filter| match filter {
                Filter::CSSFilter(options) => {
                    CSSFilter::with_options(options.clone()).apply(filtered_dom)
                }
                Filter::XPathFilter(options) => {
                    XPathFilter::with_options(options.clone()).apply(filtered_dom)
                }
                Filter::Html2TextFilter => Html2TextFilter.apply(filtered_dom),
            })
    }

    fn dom_has_changed(&self, dom: &str, other_dom: &str) -> bool {
        dom != other_dom
    }
}
