use scraper::{Html, Selector};

use crate::{
    error::WatcherError,
    model::{CSSFilterOptions, Filter, Job},
};

pub struct Watcher {}

impl Watcher {
    pub async fn run_watcher_for_job(self, job: Job) -> Result<(), WatcherError> {
        let website_dom = reqwest::get(job.url).await?.text().await?;

        let filtered_dom = self.apply_filters(website_dom, job.filters);

        Ok(())
    }

    fn apply_filters(self, dom: String, filters: Vec<Filter>) -> Result<String, WatcherError> {
        filters
            .into_iter()
            .try_fold(dom, |filtered_dom, filter| match filter {
                Filter::CSSFilter(options) => self.apply_css_filter(filtered_dom, options),
                Filter::Html2TextFilter => self.apply_html2text_filter(filtered_dom),
                Filter::XPathFilter(options) => Ok(filtered_dom),
            })
    }

    fn apply_css_filter(
        self,
        dom: String,
        options: CSSFilterOptions,
    ) -> Result<String, WatcherError> {
        let fragment = Html::parse_fragment(dom.as_str());
        let selector = Selector::parse(options.selector.as_str())?;

        let result = fragment
            .select(&selector)
            .fold(String::from(""), |acc, elem| {
                acc.push_str(elem.html().as_str());
                acc
            });

        Ok(result)
    }

    fn apply_html2text_filter(self, dom: String) -> Result<String, WatcherError> {
        let fragment = Html::parse_fragment(dom.as_str());
    }

    fn dom_has_changed(dom: String, other_dom: String) -> bool {}
}
