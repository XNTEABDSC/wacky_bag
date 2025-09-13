
pub mod grid_iter;
pub mod array_utils;
pub mod loop_wrap;
pub mod range_inclusive_upper_convert;
pub mod dim_root_of_x_usize;
//pub mod grid_iter_copy;

use crate::collections::raw_vec::RawVec;

pub fn grow_and_set<T>(list:&mut RawVec<T>,index:usize,elem:T){
    list.try_grow(index+1);
    list[index]=elem;
}
