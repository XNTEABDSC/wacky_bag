use std::ops::{Index, IndexMut};


pub trait IndexCollectionPartial<TValue> :
    Index<Self::TIndex,Output = Option<TValue>>+
    IndexMut<Self::TIndex,Output = Option<TValue>>
{
    type TIndex;
    fn add(&mut self,index:Self::TIndex,value:TValue);
    fn remove(&mut self,index:Self::TIndex);
}