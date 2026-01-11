use std::{array, mem::MaybeUninit, ops::{Deref, Div, Range, Rem}};

use crate::structures::{just::Just, n_dim_array::{NDimArray, TNDimArray, TNDimArrayParallelIterPair}, n_dim_chunk::NDimChunk, n_dim_index::{NDimIndex, NDimIndexer, TNDimIndexer}, n_dim_index_u::NDimIndexerU};

use crate::traits::scope_no_ret::{ThreadScopeCreator, ThreadScopeUser};
pub struct NDimChunkArray<const DIM:usize,T> 
{
    values:NDimArray<Just<NDimIndexerU<DIM>>,DIM,
		NDimArray<Just<NDimIndexerU<DIM>>,DIM,T,Vec<T>>,
		Vec<NDimArray<Just<NDimIndexerU<DIM>>,DIM,T,Vec<T>>>>,
	lens:[usize;DIM],
    dim_elem_count:usize,
}

impl<const DIM:usize,T> NDimChunkArray<DIM,T>
{
    pub fn from_fn(lens:[usize;DIM],dim_elem_count:usize,mut func:impl FnMut(&NDimIndex<DIM>)->T)->Self {
        // let (dim_elem_count,chunk_elem_count)=n_dim_chunk::get_chunk_dim_elem_count::<T,DIM>(n_dim_chunk::COMMON_CHUNK_SIZE);
        let lens_split_data=lens.map(|len|{
			// let chunk_count=(len as usize + dim_elem_count -1)/dim_elem_count;
			// chunk_count
			let (div,rem)=(len.div(&dim_elem_count),len.rem(&dim_elem_count));
			(div,rem)
		});
		let chunks_lens=lens_split_data.map(|(div,rem)|{if rem>0{div+1}else{div}});
		let chunks_n_dim_indexer=NDimIndexerU::new_len(chunks_lens);
		let values=NDimArray::from_fn(Just(chunks_n_dim_indexer), |t_idx|{
			let chunk_index: [usize; DIM]=array::from_fn(|i|{
				if (t_idx[i] as usize)==lens_split_data[i].0 {
					lens_split_data[i].1
				}else {
					dim_elem_count
				}
			});
			let chunk_n_dim_indexer=NDimIndexerU::new_len(chunk_index);
			let chunk_values=NDimArray::from_fn(Just(chunk_n_dim_indexer), |c_idx|{
				let full_index=array::from_fn(|i|{
					t_idx[i]*(dim_elem_count as isize) + c_idx[i]
				});
				func(&full_index)
			});
			return chunk_values;
		});
        // let n_len

        Self {
            values,
            lens,
            dim_elem_count,
        }

    }
    
}

impl<const DIM:usize,T> TNDimArray<DIM, T> for NDimChunkArray<DIM,T>
{
    fn lens(&self)->impl Deref<Target = [std::ops::Range<isize>; DIM]> {
        Just(self.lens.map(|l|0isize..(l as isize)))
    }

    fn get(&self,indexes:&super::n_dim_index::NDimIndex<DIM>)->Option<&T> {
        let split_data=indexes.map(|a|{
			let a=a as usize;
			(a/self.dim_elem_count,a%self.dim_elem_count)
		});
		let (chunks_idx,inner_idx)=(split_data.map(|(a,b)|a as isize),split_data.map(|(a,b)|b as isize));
		self.values.get(&chunks_idx).and_then(|a|a.get(&inner_idx))
    }

    fn get_mut(&mut self,indexes:&super::n_dim_index::NDimIndex<DIM>)->Option<&mut T> {
        let split_data=indexes.map(|a|{
			let a=a as usize;
			(a/self.dim_elem_count,a%self.dim_elem_count)
		});
		let (chunks_idx,inner_idx)=(split_data.map(|(a,b)|a as isize),split_data.map(|(a,b)|b as isize));
		self.values.get_mut(&chunks_idx).and_then(|a|a.get_mut(&inner_idx))
    }

    
	fn parallel_iter<'ext_env,Func,TScopeCreator>(&'ext_env self,func:&Func,scope_creator:&TScopeCreator)
			where Func: for<'scope> Fn(&'scope T,&'scope NDimIndex<DIM>)+'ext_env+Sync+Send,
			TScopeCreator: crate::traits::scope_no_ret::ThreadScopeCreator+Sync,
			T:Send+Sync {
		todo!()
	}
	
	fn parallel_iter_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:&Func,scope_creator:&TScopeCreator)
			where Func: for<'scope> Fn(&'scope mut T,&'scope NDimIndex<DIM>)+'ext_env+Sync+Send,
			TScopeCreator: crate::traits::scope_no_ret::ThreadScopeCreator+Sync,
			T:Send+Sync {
		todo!()
	}
	
	
}

impl<const DIM:usize,T> TNDimArrayParallelIterPair<DIM, T> for NDimChunkArray<DIM,T>{
	fn parallel_iter_pair<'ext_env,Func,TScopeCreator>(&'ext_env self,func:&Func,scope_creator:&TScopeCreator)
		where Func: for<'scope> Fn(&'scope T,&'scope T,usize)+'ext_env+Sync+Send,
		TScopeCreator: crate::traits::scope_no_ret::ThreadScopeCreator+Sync,
			T:Send+Sync {
		todo!()
	}
	fn parallel_iter_pair_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:&Func,scope_creator:&TScopeCreator)
        where Func: for<'scope> Fn(&'scope mut T,&'scope mut T,usize)+'ext_env+Sync+Send,
        TScopeCreator: crate::traits::scope_no_ret::ThreadScopeCreator+Sync,
            T:Send+Sync 
	{
		self.values.parallel_iter_mut(&|a,idx|{
			a.parallel_iter_pair_mut(func, scope_creator);
		}, scope_creator);
    }
	
}

pub struct NDimChunkArrayIter<'a,const DIM:usize,T>
{
	// chunk_array:&'a NDimChunkArray<DIM,T>,
	
	chunk_iter: <&'a Vec<NDimArray<Just<NDimIndexerU<DIM>>,DIM,T,Vec<T>>> as IntoIterator> ::IntoIter,
	item_iter: Option<<&'a Vec<T> as IntoIterator>::IntoIter>,
	// current_index:NDimIndex<DIM>,
	// ended:bool,
}

impl<'a,const DIM:usize,T> Iterator for NDimChunkArrayIter<'a,DIM,T>
{
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let mut item_iter=&mut self.item_iter;
		loop {
			match item_iter {
				Some(iter)=>{
					if let Some(item)=iter.next() {
						return Some(item);
					}else {
						self.item_iter=None;
					}
				},
				None=>{}
			}
			match self.chunk_iter.next() {
				Some(chunk)=>{
					self.item_iter=Some(chunk.values().iter());
					item_iter=&mut self.item_iter;
				},
				None=>{
					return None;
				}
			}
		}
	}
}

impl<'a,const DIM:usize,T> NDimChunkArray<DIM,T>
{
	pub fn iter(&'a self)->NDimChunkArrayIter<'a,DIM,T> {
		NDimChunkArrayIter {
			chunk_iter:self.values.values().iter(),
			item_iter:None,
		}
	}
}

impl<'a,const DIM:usize,T> IntoIterator for&'a NDimChunkArray<DIM,T>  {
	type Item = &'a T;

	type IntoIter = NDimChunkArrayIter<'a,DIM,T>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[cfg(test)]
mod test{
	use super::*;
	#[test]
	fn test_chunks_lens(){
		let chunk_array=NDimChunkArray::<2,i32>::from_fn([10,15],4,|idx|{
			(idx[0]*100+idx[1]) as i32
		});
		for chunks_iter in chunk_array.values.n_dim_index().iter() {
			println!("chunk idx: {:?}",chunks_iter);
			let chunk=chunk_array.values.get(&chunks_iter).unwrap();
			println!("chunk lens: {:?}",*chunk.lens());
		}
	}

	#[test]
	fn test_iter(){
		let chunk_array=NDimChunkArray::<2,i32>::from_fn([10,15],4,|idx|{
			(idx[0]*100+idx[1]) as i32
		});
		for v in chunk_array.iter() {
			println!("value: {}",*v);
		}
		
	}
	#[test]
	fn test(){
		let chunk_array=NDimChunkArray::<2,i32>::from_fn([10,15],4,|idx|{
			(idx[0]*100+idx[1]) as i32
		});
		for i in 0..10 {
			for j in 0..15 {
				assert_eq!(*chunk_array.get(&[i as isize,j as isize]).unwrap(), (i*100+j) as i32);
			}
		}
	}
}