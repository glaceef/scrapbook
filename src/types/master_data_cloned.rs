use crate::FromMasterData;

pub struct MasterDataCloned<T: Clone>(pub T);

impl<'a, T: Clone> FromMasterData<'a> for MasterDataCloned<T> {
    type MasterData = T;

    fn from_master_data(master_data: &'a Self::MasterData) -> Self {
        Self(master_data.clone())
    }
}
