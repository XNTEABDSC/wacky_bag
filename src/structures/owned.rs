//! [Owned]

use std::ops::{Deref, DerefMut};

/// A type acts like [`Deref`] smart pointers, but just own the value. allowing better generic usage.
#[derive(Debug, Clone, Copy)]
pub struct Owned<T>(pub T);

impl<T> Deref for Owned<T> {
    type Target=T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Owned<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Owned<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}