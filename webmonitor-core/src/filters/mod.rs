use async_trait::async_trait;

use crate::error::Result;

mod css;
pub use self::css::*;

mod xpath;
pub use self::xpath::*;

mod html2text;
pub use self::html2text::*;

#[async_trait]
pub trait FilterApply {
    fn apply(&self, dom: String) -> Result<String>;
}
