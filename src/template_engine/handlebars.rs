use super::TemplateEngine;
use crate::Partial;
use handlebars::Handlebars;
use serde::Serialize;

pub type Engine<'a> = Handlebars<'a>;

impl<'a> TemplateEngine for Handlebars<'a> {
    fn new() -> Self {
        Handlebars::new()
    }

    fn render<T: Serialize>(&self, partial: Partial, data: T) -> String {
        let data2 = serde_json::to_string_pretty(&data).unwrap();
        println!("{data2}");

        self.render(&partial, &data).unwrap()
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! register_helper {
        ($handlebars:expr, $helper:ident) => {
            $handlebars.register_helper(stringify!($helper), Box::new(helpers::$helper));
        };
    }

    pub use handlebars::handlebars_helper;
    pub use register_helper;
}
