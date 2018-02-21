extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invert<T: Ord>(T);

impl<T: Ord> PartialOrd for Invert<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl<T: Ord> Ord for Invert<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: Ord> Invert<T> {
    pub fn new(t: T) -> Self {
        Invert(t)
    }
    pub fn inner(&self) -> &T {
        &self.0
    }
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub fn invert<T: Ord>(t: T) -> Invert<T> {
    Invert::new(t)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialInvert<T: PartialOrd>(T);

impl<T: PartialOrd> PartialOrd for PartialInvert<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialInvert<T> {
    pub fn new(t: T) -> Self {
        PartialInvert(t)
    }
    pub fn inner(&self) -> &T {
        &self.0
    }
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub fn partial_invert<T: PartialOrd>(t: T) -> PartialInvert<T> {
    PartialInvert::new(t)
}
