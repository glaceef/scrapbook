use crate::Partial;
use serde::Serialize;

#[cfg(feature = "handlebars")]
#[cfg_attr(docsrs, doc(cfg(feature = "handlebars")))]
#[doc = "Default tempalte engine"]
pub mod handlebars;

#[cfg(feature = "handlebars")]
pub use self::handlebars::Engine;

pub trait TemplateEngine: Sized {
    fn new<'a>() -> Self;

    fn render<T: Serialize>(&self, partial: Partial, data: T) -> String;
}
