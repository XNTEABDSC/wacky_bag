use std::ops::{Deref, DerefMut};



pub struct Just<T>(pub T);

impl<T> Deref for Just<T> {
    type Target=T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<T> DerefMut for Just<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}