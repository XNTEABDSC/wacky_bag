
pub mod grid_iter;

use crate::collections::raw_vec::RawVec;

pub fn grow_and_set<T>(list:&mut RawVec<T>,index:usize,elem:T){
    list.try_grow(index+1);
    list[index]=elem;
}
