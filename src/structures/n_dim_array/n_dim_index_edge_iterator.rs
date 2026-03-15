use std::{array, ops::Deref};

use crate::structures::n_dim_array::{dim_dir::DimDir, n_dim_index::NDimIndex, n_dim_index_operator::NDimIndexOperator, t_n_dim_indexer::TNDimIndexer};

pub struct NDimIndexEdgeIterator<const DIM:usize, TIndexer>{
	operator:NDimIndexOperator<DIM,TIndexer>,
	dim_dir:DimDir,
	ended:bool,
	dim_restricted:isize,
}


impl<const DIM:usize, TIndexer> NDimIndexEdgeIterator<DIM,TIndexer> 
	where TIndexer: Deref<Target : TNDimIndexer<DIM>>
{
	pub fn new(indexer:TIndexer,dim_dir:DimDir)->Self{
		let dim=dim_dir.dim;
		let dim_restricted=if dim_dir.dir_positive {
			indexer.lens()[dim].end-1
		} else {
			0
		};

		let operator=NDimIndexOperator::from_index(indexer, array::from_fn::<_,DIM,_>(|d|{
			if d==dim{dim_restricted} else {0}
		})).unwrap();
		Self { operator, dim_dir, ended: false, dim_restricted }
	}
}

impl<const DIM:usize, TIndexer> Iterator for NDimIndexEdgeIterator<DIM,TIndexer> 
	where TIndexer: Deref<Target : TNDimIndexer<DIM>>
{
	type Item=(NDimIndex<DIM>,usize);

	fn next(&mut self) -> Option<Self::Item> {
		if self.ended {
			return None;
		}
		let operator=&mut self.operator;
		let dim=self.dim_dir.dim;

		if self.dim_dir.dir_positive {
			let c=operator.move_n_carry(1);
			if c!=0 {return None;}// iterating is over

			if operator.get()[dim]==0 {// a idx change at mid
				operator.set_n_at_dim(dim, self.dim_restricted);
			}
		}else {
			let c=operator.move_n_carry(1);
			if c!=0 {// iterating is over
				return None;
			}
			if operator.get()[dim]!=0 {// b idx change at mid
				operator.set_n_at_dim(dim, 0);
				if 1<=dim {
					let c=operator.move_n_carry_at_dim(dim-1, 1);
					if c!=0 {// iterating is over
						return None;
					}
				}else {// iterating is over
					return None;
				}
			}
		}
		return Some((self.operator.get().clone(),self.operator.get_compressed()));
	}
}