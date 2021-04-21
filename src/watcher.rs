use std::sync::Arc;

use scraper::{Html, Selector};

use crate::{
    database::DatabaseAdapter,
    error::WatcherError,
    model::{CSSFilterOptions, Filter, Job},
};

pub struct Watcher {
    db: Arc<DatabaseAdapter>,
}

impl Watcher {
    pub fn new(db: Arc<DatabaseAdapter>) -> Self {
        Self { db }
    }

    pub async fn run_watcher_for_job(&self, job: &Job) -> Result<(), WatcherError> {
        let website_dom = reqwest::get(&job.url).await?.text().await?;

        let filtered_dom = self.apply_filters(website_dom, &job.filters)?;

        let last_snapshot = self.db.snapshots_get_latest(&job.id).await?;

        if last_snapshot.is_none()
            || self.dom_has_changed(&last_snapshot.unwrap().data, &filtered_dom)
        {
            println!("Job {} has changed!", &job.name)
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
        dom == other_dom
    }
}
