
pub mod grid_iter;
pub mod array_utils;
pub mod loop_wrap;
pub mod range_inclusive_upper_convert;
pub mod dim_root_of_x_usize;
pub mod output_func;
pub mod type_fn;
pub mod select_zip;
pub mod h_list_helpers;
pub mod impl_phantom;
pub mod default_of;
pub mod num_extend;
pub mod mul_as_add;
pub mod h_extend_by_fn;
// pub mod h_type_mappable;
//pub mod grid_iter_copy;

use crate::collections::raw_vec::RawVec;

pub fn grow_and_set<T>(list:&mut RawVec<T>,index:usize,elem:T){
    list.try_grow(index+1);
    list[index]=elem;
}
