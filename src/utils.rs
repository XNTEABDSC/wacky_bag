
use crate::collections::raw_vec::RawVec;

pub fn add_to_and_set<T>(list:&mut RawVec<T>,index:usize,elem:T){
    list.try_grow(index+1);
    list[index]=elem;
}
