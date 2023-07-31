//! TODO: このクレートの説明

// #![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod extract;
mod partial;
mod section;
mod section_data;
mod template_engine;
mod types;

pub mod engine {
    pub use super::template_engine::*;
}

pub use engine::{Engine, TemplateEngine};
pub use extract::FromMasterData;
pub use partial::Partial;
pub use section::{Section, Sections};
pub use section_data::SectionData;
pub use types::*;

pub struct Scrapbook<MD> {
    master_data: MD,
}

impl<MD> Scrapbook<MD> {
    pub fn new(master_data: MD) -> Self {
        Self { master_data }
    }

    pub fn render<'a, E, T>(&'a self, init: fn(&mut E)) -> String
    where
        E: TemplateEngine,
        T: SectionData<'a>,
        <T as SectionData<'a>>::Data: FromMasterData<'a, MasterData = MD>,
    {
        let partial = <T as SectionData>::PARTIAL.into();

        let data = <T as SectionData>::Data::from_master_data(&self.master_data);
        let section_data = <T as SectionData>::section_data(&partial, data);

        let mut engine = <E as TemplateEngine>::new();
        init(&mut engine);

        E::render(engine, partial, section_data)
    }
}
