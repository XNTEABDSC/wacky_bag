use std::ops::{ControlFlow, Deref};

use crate::{structures::n_dim_index::{NDimIndex, TNDimIndexer}, utils::loop_wrap::loop_wrap_assign};


#[derive(Clone, Copy)]
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
		let step_at_dim=self.indexer.steps()[dim_idx];
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
		let mut compressed=self.compressed as isize + n*step_at_dim as isize;
		// self.compressed = (self.compressed as isize + n*step_at_dim as isize) as usize;
		let c2=loop_wrap_assign(&mut compressed, & (0..*self.indexer.length() as isize), *self.indexer.length() as isize);
		self.compressed=compressed as usize;
		debug_assert_eq!(c2,match res {
			ControlFlow::Continue(c)=>c,
			ControlFlow::Break(_)=>0
		});

		return c2;
	}
	pub fn move_n_at_dim(&mut self,dim_idx:usize,mut n:isize)->isize{
		let range=&self.indexer.lens()[dim_idx];
		let range_len=range.end-range.start;
		let index_at_dim=self.index[dim_idx];
		let step_at_dim=self.indexer.steps()[dim_idx];
		let mut new_index_at_dim=index_at_dim+n;
		let c=loop_wrap_assign(&mut new_index_at_dim, range, range_len);
		n -= c*range_len;
		self.compressed = ( self.compressed as isize + n * (step_at_dim as isize)) as usize;
		self.index[dim_idx]=new_index_at_dim;
		return c;
	}
	pub fn set_n_at_dim(&mut self,dim_idx:usize,n:isize){
		// self.move_n_at_dim(dim_idx, n-(self.index[dim_idx] as isize))
		let range=&self.indexer.lens()[dim_idx];
		debug_assert!(range.contains(&n));
		let index_at_dim=self.index[dim_idx];
		let step_at_dim=self.indexer.steps()[dim_idx];
		let index_delta=n-index_at_dim;
		let compressed=self.compressed as isize;
		let new_compressed=compressed + step_at_dim as isize * index_delta;
		self.compressed=new_compressed as usize;
		self.index[dim_idx]=n;
		return;
	}
}

#[cfg(test)]
mod test{
	use crate::structures::n_dim_index::{NDimIndexer};

	use super::*;
	#[test]
	fn test1(){
		let a_idxer=NDimIndexer::new_len([0..5,0..5,0..5]);

		let check=|op: &NDimIndexOperator<3, &NDimIndexer<3>>,b|{
			assert_eq!(op.get(),b);
			assert_eq!(op.get_compressed(),a_idxer.compress_index(b));
		};

		let a_index=[1,2,3];
		let mut a_idx_op=NDimIndexOperator::from_index(&a_idxer, a_index.clone()).unwrap();
		check(&a_idx_op,&a_index);

		a_idx_op.move_n_carry(2);
		check(&a_idx_op,&[1,3,0]);

		assert_eq!(a_idx_op.move_n_at_dim(1, -4),-1);
		check(&a_idx_op,&[1,4,0]);

		a_idx_op.set_n_at_dim(1, 2);
		check(&a_idx_op,&[1,2,0]);

		assert_eq!(a_idx_op.move_n_carry_at_dim(0, 9),2);
		check(&a_idx_op,&[0,2,0]);

	}
}