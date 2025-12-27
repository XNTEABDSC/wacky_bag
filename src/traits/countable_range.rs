use std::{iter::Peekable, marker::PhantomData, ops::Deref};


pub trait CountableRangeItemN:CountableRangeItem {
    /// move self for n step.
    /// if overflow, it should loop and return times of loops (negative if loop back).
    fn move_n_assign(&mut self,n:isize)->isize;
}

/// Items that can be iterated and be mapped to usize
pub trait CountableRangeItem
    //where Self:Sized
{
    /// `CountableRangeForRef` that returned by `range()`
    type RangeRef<'a>:CountableRangeForRef where 
        Self:'a,;
    //fn next(self)->Option<Self>;


    /// move self to its next state. 
    /// 
    /// if it has no next state (aka owerflow), it should be set to first state and return true.
    /// 
    /// else return false.
    fn next_assign(&mut self)->bool;


    /// move self to its prev state. 
    /// 
    /// if it has no prev state (aka owerflow), it should be set to last state and return false.
    /// 
    /// else return false.
    fn prev_assign(&mut self)->bool;


    /// set self to the first state
    fn first_assign(&mut self);

    /// set self to the last state
    fn last_assign(&mut self);

    /// get index of self
    fn index(&self)->usize;

    /// get range of self
    fn range<'a>(&'a self)->Self::RangeRef<'a>;

    
}

/// Range of some indexable items, useful for many to be merged into a single index
/// 
/// You can use
/// ```
/// impl<T,RangeRef> CountableRange for RangeRef
/// where 
///     RangeRef:Deref<Target = YourStruct<T>>+Clone,
/// ```
/// To make them
pub trait CountableRangeForRef{
    type Item:CountableRangeItem;
    fn item_first(self)->Self::Item;
    fn item_last(self)->Self::Item;
    fn item_from_index(self,index:usize)->Option<Self::Item>;
    fn index_len(&self)->usize;
}

pub struct CountableRangeItemIterator<T:CountableRangeItem+Copy>{
    item:Option<T>,
}

impl<T:CountableRangeItem+Copy> CountableRangeItemIterator<T> {
    pub fn new(item:T)->Self{Self{
        item:Some(item),
    }}
}
impl<T:CountableRangeItem+Copy> Iterator for CountableRangeItemIterator<T>  {
    type Item=T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item)=&mut self.item{
            let a=*item;
            let ended=item.next_assign();
            if ended{
                self.item=None;
                
            }else {
                
            }
            Some(a)
            // let result=Some(item);
            // let next=item.next();
            // self.item=next;
            // return result;
        }else {
            return None;
        }
    }
}