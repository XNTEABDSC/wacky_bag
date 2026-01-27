use std::ops::{Deref, Range};

use crate::structures::n_dim_array::n_dim_index::NDimIndex;

pub trait TNDimIndexer<const DIM:usize> {
    fn length(&self) -> impl Deref<Target=usize>;
    fn lens(&self)->impl Deref<Target=[Range<isize>;DIM]>;
	fn steps(&self)->&[usize;DIM];
    // fn length(&self)->&Range<isize>;

    
    fn contains(&self,indexes:&NDimIndex<DIM>)->bool;

    fn contains_compressed(&self,index:usize)->bool;
    
    fn compress_index(&self,indexes:&NDimIndex<DIM>)->usize;
    fn decompress_index(&self,compressed_index:usize)->NDimIndex<DIM>;
	fn decompress_index_at_dim(&self,compressed_index:usize,dim:usize)->isize;
	fn add_index_at_dim(&self,compressed_index:usize,dim:usize,add_index:isize)->usize;
    fn iter<'a>(&'a self)->impl Iterator<Item=NDimIndex<DIM>> + 'a;
}