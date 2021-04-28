use async_trait::async_trait;

use crate::{error::Result, model::XPathFilterOptions};

use super::FilterApply;

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
    fn apply(&self, dom: String) -> Result<String> {
        Ok(dom)
    }
}
