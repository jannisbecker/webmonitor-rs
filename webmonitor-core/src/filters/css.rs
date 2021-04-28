use async_trait::async_trait;
use scraper::{Html, Selector};

use crate::{error::FilterError, model::CSSFilterOptions};

use super::FilterApply;

pub struct CSSFilter {
    options: CSSFilterOptions,
}

impl CSSFilter {
    pub fn with_options(options: CSSFilterOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl FilterApply for CSSFilter {
    fn apply(&self, dom: String) -> Result<String, FilterError> {
        let fragment = Html::parse_fragment(dom.as_str());
        let selector = Selector::parse(self.options.selector.as_str())
            .map_err(|_e| FilterError::SelectorParseError)?;

        let result = fragment
            .select(&selector)
            .fold(String::from(""), |mut acc, elem| {
                acc.push_str(elem.html().as_str());
                acc
            });

        Ok(result)
    }
}
