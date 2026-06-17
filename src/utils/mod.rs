//! utils
pub mod grid_iter;
// pub mod array_utils;
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
// pub mod mul_as_add;
pub mod h_extend_by_fn;
pub mod factorial;
pub mod d_sphere_volume;
pub mod rw_lock_error_either;
pub mod h_h_zippable;
pub mod phantom_data_type_params;


// use crate::collections::raw_vec::RawVec;

// pub fn grow_and_set<T>(list:&mut RawVec<T>,index:usize,elem:T){
//     list.try_grow(index+1);
//     list[index]=elem;
// }

/// showing that F is `impl FnOnce(I)->O`
pub fn restrict_fn_once_type<F,I,O>(f:F)->F
where F:FnOnce(I)->O
{
	f
}