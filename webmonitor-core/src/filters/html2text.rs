use async_trait::async_trait;
use scraper::Html;

use crate::error::FilterError;

use super::FilterApply;

pub struct Html2TextFilter;

#[async_trait]
impl FilterApply for Html2TextFilter {
    fn apply(&self, dom: String) -> Result<String, FilterError> {
        let fragment = Html::parse_fragment(dom.as_str());
        let result = fragment.root_element().text().collect();

        Ok(result)
    }
}
