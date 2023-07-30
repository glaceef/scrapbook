use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Partial(String);

impl From<&Partial> for Partial {
    fn from(partial_ref: &Partial) -> Self {
        partial_ref.clone()
    }
}

impl<T: Display> From<T> for Partial {
    fn from(value: T) -> Self {
        Partial(value.to_string())
    }
}

impl Deref for Partial {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Partial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for Partial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + &rhs.0)
    }
}

impl AddAssign for Partial {
    fn add_assign(&mut self, other: Self) {
        self.0 += &other.0;
    }
}

impl Partial {
    pub fn new(partial: impl Into<String>) -> Self {
        Self(partial.into())
    }

    pub fn join(&self, partial: impl Into<Partial>) -> Self {
        Partial(format!("{}/{}", self.0, partial.into().0))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
