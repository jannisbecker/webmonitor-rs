use std::sync::Arc;

use futures::future;
use scraper::{Html, Selector};

use crate::{
    error::WatcherError,
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
                        DiscordNotification::from_options(options.clone())
                            .send(job, &prev_snap, &new_snap)
                            .await
                    }
                    Notification::Email(options) => {
                        EmailNotification::from_options(options.clone())
                            .send(job, &prev_snap, &new_snap)
                            .await
                    }
                };
            }))
            .await;
        }

        Ok(())
    }

    fn apply_filters(&self, dom: String, filters: &Vec<Filter>) -> Result<String, WatcherError> {
        filters
            .into_iter()
            .try_fold(dom, |filtered_dom, filter| match filter {
                Filter::CSSFilter(options) => self.apply_css_filter(filtered_dom, options),
                Filter::Html2TextFilter => self.apply_html2text_filter(filtered_dom),
                Filter::XPathFilter(options) => Ok(filtered_dom),
            })
    }

    fn apply_css_filter(
        &self,
        dom: String,
        options: &CSSFilterOptions,
    ) -> Result<String, WatcherError> {
        let fragment = Html::parse_fragment(dom.as_str());
        let selector = Selector::parse(options.selector.as_str())
            .map_err(|_e| WatcherError::SelectorParseError)?;

        let result = fragment
            .select(&selector)
            .fold(String::from(""), |mut acc, elem| {
                acc.push_str(elem.html().as_str());
                acc
            });

        Ok(result)
    }

    fn apply_html2text_filter(&self, dom: String) -> Result<String, WatcherError> {
        let fragment = Html::parse_fragment(dom.as_str());
        let result = fragment.root_element().text().collect();

        Ok(result)
    }

    fn dom_has_changed(&self, dom: &str, other_dom: &str) -> bool {
        dom != other_dom
    }
}
