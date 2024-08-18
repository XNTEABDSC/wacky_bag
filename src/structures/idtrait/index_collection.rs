use std::ops::{Index, IndexMut};


pub trait IndexCollection<TValue> :
    IntoIterator<Item=(Self::TIndex,TValue)>+
    Index<Self::TIndex,Output = TValue>+
    IndexMut<Self::TIndex,Output = TValue>
{
    type TIndex;
    fn add(&mut self,index:Self::TIndex,value:TValue);
    fn remove(&mut self,index:Self::TIndex);
}

