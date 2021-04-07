use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub url: String,
    pub interval: i16,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Filter {
    CSSFilter(CSSFilterOptions),
    XPathFilter(XPathFilterOptions),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CSSFilterOptions {
    pub selector: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XPathFilterOptions {
    pub selector: String,
}
