use crate::Partial;

use super::FromMasterData;
use serde::Serialize;
use std::fmt::Display;

pub trait SectionData<'a>: Serialize {
    type Partial: Display;
    type Data: FromMasterData<'a>;

    const PARTIAL: Self::Partial;

    // init とかでもよいかも
    fn section_data(partial: &Partial, data: Self::Data) -> Self;

    fn should_skip(_master_data: &<Self::Data as FromMasterData<'a>>::MasterData) -> bool {
        false
    }
}
