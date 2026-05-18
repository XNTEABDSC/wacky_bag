//! trait for NDimIndexer
//! [`TNDimIndexer`]

use std::ops::{Deref, Range};

use crate::structures::n_dim_array::{dim_dir::DimDir, n_dim_index::NDimIndex, n_dim_index_edge_iterator::NDimIndexEdgeIterator, n_dim_index_iterator::NDimIndexIter};

/// trait for NDimIndexer, mapping between [`NDimIndex`] and compressed [`usize`] index for NDimArray
pub trait TNDimIndexer<const DIM:usize> {
	/// 0..length range of compressed index
    fn length(&self) -> impl Deref<Target=usize>;
	/// range of NDimIndex
    fn lens(&self)->impl Deref<Target=[Range<isize>;DIM]>;
	/// steps for each index at dim
	fn steps(&self)->&[usize;DIM];
	

    /// check whether [`NDimIndex`] is inside
    fn contains(&self,indexes:&NDimIndex<DIM>)->bool;

    /// check whether compressed index is inside
    fn contains_compressed(&self,index:usize)->bool;
    
	/// converts [`NDimIndex`] to compressed [`usize`]
    fn compress_index(&self,indexes:&NDimIndex<DIM>)->usize;
	/// compressed [`usize`] to converts [`NDimIndex`] 
    fn decompress_index(&self,compressed_index:usize)->NDimIndex<DIM>;
	/// get then index at dim of compressed [`usize`]
	fn decompress_index_at_dim(&self,compressed_index:usize,dim:usize)->isize;
	/// for compressed [`usize`], add `add_index` at `dim`
	fn add_index_at_dim(&self,compressed_index:usize,dim:usize,add_index:isize)->usize;
	/// iterate all indexes.
    fn iter<'a>(&'a self)->impl Iterator<Item=NDimIndex<DIM>> + 'a{
		NDimIndexIter::<DIM,_>::new(self.lens())
	}
	/// iterate all indexes at edge.
	fn edge_iter<'a>(&'a self,dim_dir:DimDir)->impl Iterator<Item=(NDimIndex<DIM>,usize)>+'a{
		// let op=NDimIndexOperator::new(indexer, index, compressed)
		NDimIndexEdgeIterator::new(self, dim_dir)
	}
}