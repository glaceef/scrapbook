use crate::FromMasterData;

pub struct MasterDataRef<'a, T>(pub &'a T);

impl<'a, T> FromMasterData<'a> for MasterDataRef<'a, T> {
    type MasterData = T;

    fn from_master_data(master_data: &'a Self::MasterData) -> Self {
        Self(master_data)
    }
}
