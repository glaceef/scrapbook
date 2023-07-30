use crate::{FromMasterData, Partial, SectionData};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Section {
    pub partial: Partial,
    pub data: serde_json::Value,
    pub number: u32,
}

#[derive(Clone, Debug)]
pub struct Sections<'a, MD> {
    master_data_ref: &'a MD,
    current_partial: Partial,
    sections: Vec<Section>,
}

// D: Serialize を不要にする。
// また、セクションデータがSections型のフィールドを定義するとき、
// #[serde(flatten)] の記述を不要にする。
impl<'a, MD> Serialize for Sections<'a, MD> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.sections.serialize(serializer)
    }
}

impl<'a, MD> Sections<'a, MD> {
    pub fn new(master_data_ref: &'a MD, partial: impl Into<Partial>) -> Self {
        Self {
            sections: vec![],
            master_data_ref,
            current_partial: partial.into(),
        }
    }

    pub fn append<'b, T>(&'b mut self) -> &mut Self
    where
        T: SectionData<'a>,
        <T as SectionData<'a>>::Data: FromMasterData<'a, MasterData = MD>,
    {
        if T::should_skip(self.master_data_ref) {
            return self;
        }

        let partial = self.current_partial.join(<T as SectionData>::PARTIAL);

        let data = <T as SectionData>::Data::from_master_data(self.master_data_ref);
        let section_data = T::section_data(&partial, data);

        self.sections.push(Section {
            partial,
            number: (self.sections.len() + 1) as u32,
            data: serde_json::to_value(section_data).unwrap(),
        });

        self
    }
}
