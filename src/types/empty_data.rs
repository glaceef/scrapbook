use crate::FromMasterData;
use std::marker::PhantomData;

pub struct EmptyData<T>(PhantomData<T>);

impl<'a, T> FromMasterData<'a> for EmptyData<T> {
    type MasterData = T;

    fn from_master_data(_: &'a Self::MasterData) -> Self {
        EmptyData(PhantomData)
    }
}
