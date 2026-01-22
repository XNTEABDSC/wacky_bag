use std::ops::{ControlFlow, Deref};

use crate::{structures::n_dim_index::{NDimIndex, TNDimIndexer}, utils::loop_wrap::loop_wrap_assign};



pub struct NDimIndexOperator<const DIM:usize,TIndexer>{
	indexer:TIndexer,
	index:NDimIndex<DIM>,
	compressed:usize
}

impl<const DIM:usize,TIndexer> NDimIndexOperator<DIM,TIndexer>
	where TIndexer: Deref<Target : TNDimIndexer<DIM>>
{
	pub unsafe fn new(indexer:TIndexer,index:NDimIndex<DIM>,compressed:usize)->Self{
		NDimIndexOperator{
			index,indexer,compressed
		}
	}

	pub fn from_index(indexer:TIndexer,index:NDimIndex<DIM>)->Option<Self>{
		if !indexer.contains(&index){
			return None;
		}
		let compressed=indexer.compress_index(&index);
		return Some(NDimIndexOperator{
			index,indexer,compressed
		});
	}

	pub fn from_compressed(indexer:TIndexer,compressed:usize)->Option<Self>{
		if !indexer.contains_compressed(compressed) {
			return None;
		}
		let index=indexer.decompress_index(compressed);
		return Some(NDimIndexOperator{
			index,indexer,compressed
		});
	}

	pub fn get(&self)->&NDimIndex<DIM>{
		&self.index
	}

	pub fn get_compressed(&self)->usize{
		self.compressed
	}

	pub fn move_n_carry(&mut self,n:isize)->isize{
		self.move_n_carry_at_dim(DIM-1, n)
	}
	pub fn move_n_carry_at_dim(&mut self,dim_idx:usize,n:isize)->isize{
		let res=self.index.iter_mut().zip(
			self.indexer.lens().iter()
		).rev().skip(DIM-dim_idx-1).try_fold(n, |c,idx|{
			*idx.0+=c;
			let n=loop_wrap_assign(idx.0, idx.1, idx.1.end-idx.1.start) as isize;
			if n==0 {
				return ControlFlow::Break(());
			}else {
				return ControlFlow::Continue(n);
			}
		});
		self.compressed = (self.compressed as isize + n) as usize;
		let c2=loop_wrap_assign(&mut self.compressed, & (0..*self.indexer.length()), *self.indexer.length());
		
		debug_assert_eq!(c2,match res {
			ControlFlow::Continue(c)=>c,
			ControlFlow::Break(_)=>0
		});

		return c2;
		// return match res {
		// 	ControlFlow::Continue(c) => {
		// 		debug_assert_eq!(c,c2);
		// 	},
		// 	ControlFlow::Break(_) => {
		// 		debug_assert_eq!(c,c2);
		// 	},
		// };
	}
	pub fn move_n_at_dim(&mut self,dim_idx:usize,mut n:isize)->isize{
		let range=&self.indexer.lens()[dim_idx];
		let dim_len=range.end-range.start;
		self.index[dim_idx]+=n;
		let c=loop_wrap_assign(&mut self.index[dim_idx], range, dim_len);
		n -= c*dim_len;
		self.compressed = ( self.compressed as isize + n * (self.indexer.steps()[dim_idx] as isize)) as usize;
		return c;
	}
}

#[cfg(test)]
mod test{
	use crate::structures::n_dim_index::{NDimIndexer};

	use super::*;
	#[test]
	fn test1(){
		let a_idxer=NDimIndexer::new_len([0..5,0..5,0..5]);
		let a_index=[1,2,3];
		let mut a_idx_op=NDimIndexOperator::from_index(&a_idxer, a_index.clone()).unwrap();
		assert_eq!(a_idx_op.compressed,a_idxer.compress_index(&a_index));
		let a_index_2=[1,3,0];
		a_idx_op.move_n_carry(2);
		assert_eq!(a_idx_op.get(),&a_index_2);
		assert_eq!(a_idx_op.get_compressed(),a_idxer.compress_index(&a_index_2));
		let a_index_3=[1,4,0];
		assert_eq!(a_idx_op.move_n_at_dim(1, -4),-1);
		assert_eq!(a_idx_op.get(),&a_index_3);
		assert_eq!(a_idx_op.get_compressed(),a_idxer.compress_index(&a_index_3));

	}
}