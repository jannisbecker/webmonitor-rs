use async_trait::async_trait;
use scraper::{Html, Selector};

use crate::{
    error::FilterError,
    model::{CSSFilterOptions, XPathFilterOptions},
};

#[async_trait]
pub trait FilterApply {
    fn apply(&self, dom: String) -> Result<String, FilterError>;
}

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

pub struct XPathFilter {
    options: XPathFilterOptions,
}
impl XPathFilter {
    pub fn with_options(options: XPathFilterOptions) -> Self {
        Self { options }
    }
}
#[async_trait]
impl FilterApply for XPathFilter {
    fn apply(&self, dom: String) -> Result<String, FilterError> {
        Ok(dom)
    }
}

pub struct Html2TextFilter;
#[async_trait]
impl FilterApply for Html2TextFilter {
    fn apply(&self, dom: String) -> Result<String, FilterError> {
        let fragment = Html::parse_fragment(dom.as_str());
        let result = fragment.root_element().text().collect();

        Ok(result)
    }
}
