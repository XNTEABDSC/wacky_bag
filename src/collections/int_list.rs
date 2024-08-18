use std::ops::{Index, IndexMut};

use crate::structures::idtrait::index_collection_parial::IndexCollectionPartial;

pub struct IntList<T>{
    list:Vec<Option<T>>
}

impl <T> IntList<T> {
    pub fn new()->Self{
        IntList::<T>{
            list:Vec::new()
        }
    }
}

impl <T> Index<usize> for IntList<T> {
    type Output=Option<T>;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.list.index(index)
    }
}

impl <T> IndexMut<usize> for IntList<T> {
    //type Output=Option<T>;

    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.list.index_mut(index)
    }
}

impl<T> IndexCollectionPartial<T> for IntList<T> {
    type TIndex=usize;
    #[inline]
    fn add(&mut self,index:Self::TIndex,value:T){
        while self.list.len()<=index {
            self.list.push(Option::None)
        }
        self.list[index]=Option::Some(value)
    }
    #[inline]
    fn remove(&mut self,index:Self::TIndex){
        if self.list.len()>=index {
            self.list[index]=Option::None
        }
    }
}
