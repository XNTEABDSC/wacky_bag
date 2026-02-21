use std::{array, mem::transmute, ops::{Deref, Div, Rem}};

use crate::structures::{just::Just, n_dim_array::{n_dim_array::NDimArray, n_dim_index::NDimIndex, n_dim_index_operator::NDimIndexOperator, n_dim_indexer_u::NDimIndexerU, t_n_dim_array::{TNDimArray,TNDimArrayIterPair,TNDimArrayParallelIterPair}}};

pub struct NDimChunkArray<const DIM:usize,T> 
{
    values:NDimArray<Just<NDimIndexerU<DIM>>,DIM,
		NDimArray<Just<NDimIndexerU<DIM>>,DIM,T,Vec<T>>,
		Vec<NDimArray<Just<NDimIndexerU<DIM>>,DIM,T,Vec<T>>>>,
	lens:[usize;DIM],
    dim_elem_count:usize,
}

fn merge_index<const DIM:usize>(chunks_idx:&NDimIndex<DIM>,elem_idx:&NDimIndex<DIM>,dim_elem_count:usize)->NDimIndex<DIM>{
	array::from_fn(|dim|{chunks_idx[dim]*(dim_elem_count as isize)+elem_idx[dim]})
}
fn split_index<const DIM:usize>(indexes:&NDimIndex<DIM>,dim_elem_count:usize)->(NDimIndex<DIM>, NDimIndex<DIM>){
	let split_data=indexes.map(|a|{
		let a=a as usize;
		(a/dim_elem_count,a%dim_elem_count)
	});
	let (chunks_idx,inner_idx)=(split_data.map(|(a,_)|a as isize),split_data.map(|(_,b)|b as isize));
	return (chunks_idx,inner_idx); 

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
		let (chunks_idx,inner_idx)=split_index(indexes, self.dim_elem_count);
		self.values.get(&chunks_idx).and_then(|a|a.get(&inner_idx))
    }

    fn get_mut(&mut self,indexes:&super::n_dim_index::NDimIndex<DIM>)->Option<&mut T> {
		let (chunks_idx,inner_idx)=split_index(indexes, self.dim_elem_count);
		self.values.get_mut(&chunks_idx).and_then(|a|a.get_mut(&inner_idx))
    }

    
	fn for_each_parallel<'ext_env,Func,TScopeCreator>(&'ext_env self,func:&Func,scope_creator:&TScopeCreator)
			where Func: for<'scope> Fn(&'scope T,NDimIndex<DIM>)+'ext_env+Sync+Send,
			TScopeCreator: crate::traits::scope_no_ret::ThreadScopeCreator+Sync,
			T:Send+Sync {
		self.values.for_each_parallel(&|a,chunks_idx|{
			a.for_each(&|b,elem_idx|{
				let idx=merge_index(&chunks_idx, &elem_idx, self.dim_elem_count);
				func(b,idx);
			});
		}, scope_creator);
	}
	
	fn for_each_mut_parallel<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:&Func,scope_creator:&TScopeCreator)
			where Func: for<'scope> Fn(&'scope mut T,NDimIndex<DIM>)+'ext_env+Sync+Send,
			TScopeCreator: crate::traits::scope_no_ret::ThreadScopeCreator+Sync,
			T:Send+Sync {
		self.values.for_each_mut_parallel(&|chunk,chunks_idx|{
			chunk.for_each_mut(&|item,elem_idx|{
				let idx=merge_index(&chunks_idx, &elem_idx, self.dim_elem_count);
				func(item,idx);
			});
		}, scope_creator);
	}
	
	fn for_each<'env, Func>(&'env self, func: &Func)
		where
			Func: Fn(&'env T, NDimIndex<DIM>),
				T:'env {
					self.values.for_each(&|a,chunks_idx|{
						a.for_each(&|b,elem_idx|{
							let idx=merge_index(&chunks_idx, &elem_idx, self.dim_elem_count);
							func(b,idx);
						});
					});
		}
	
	fn for_each_mut<'env, Func>(&'env mut self, func: &Func)
			where
				Func: Fn(&'env mut T, NDimIndex<DIM>),
				T:'env {
					self.values.for_each_mut(&|chunk,chunks_idx|{
						chunk.for_each_mut(&|item,elem_idx|{
							let idx=merge_index(&chunks_idx, &elem_idx, self.dim_elem_count);
							func(item,idx);
						});
					});
		}
	
	
}

impl<const DIM:usize,T> TNDimArrayIterPair<DIM, T> for NDimChunkArray<DIM,T>
{
	fn iter_pair<'ext_env, Func>(
		&'ext_env self,
		func: &Func,
	) where
		Func: for<'scope> Fn(&'scope T, &'scope T, usize) + 'ext_env + Sync + Send,
		T: Send + Sync {
		self.values.for_each(&|chunk,_|{
			chunk.iter_pair(&|a,b,dim|{
				func(a,b,dim);
			});
		});
		self.values.iter_pair(&|chunka,chunkb,dim|{
			let a_uw=chunka.unwrap_ref();
			let b_uw=chunkb.unwrap_ref();
			let a_end_i=a_uw.0.lens()[dim] as isize-1;
			let mut a_idx_op=NDimIndexOperator::from_index(a_uw.0.deref(), array::from_fn::<_,DIM,_>(|d|{
				if d==dim{a_end_i} else {0}
			})).unwrap();
			let mut b_idx_op = NDimIndexOperator::from_index(b_uw.0.deref(), [0;DIM]).unwrap();
			
			loop {
				let a_item=unsafe {
					transmute(&a_uw.1[a_idx_op.get_compressed()])
				};
				let b_item=unsafe {
					transmute(&b_uw.1[b_idx_op.get_compressed()])
				};
				func(a_item,b_item,dim);
				// func(&mut a_uw.1[a_idx_op.get_compressed()],&mut b_uw.1[b_idx_op.get_compressed()],dim);
				let c=a_idx_op.move_n_carry(1);
				if c!=0 {break;}// iterating is over
				let mut dim_carry=false;

				if a_idx_op.get()[dim]==0 {// a idx change at mid
					a_idx_op.set_n_at_dim(dim, a_end_i);
					if cfg!(debug_assertions){
						dim_carry=true;
					}
				}
				let c=b_idx_op.move_n_carry(1);
				if c!=0 {// iterating is over
					debug_assert!(false,"not synchronous carry of 2 index iterating");
					break;
				}
				if b_idx_op.get()[dim]!=0 {// b idx change at mid
					b_idx_op.set_n_at_dim(dim, 0);
					if 1<=dim {
						let c=b_idx_op.move_n_carry_at_dim(dim-1, 1);
						if c!=0 {// iterating is over
							debug_assert_eq!(dim_carry,true,"not synchronous carry of 2 index iterating");
							break;
						}
					}else {// iterating is over
						debug_assert!(false,"not synchronous carry of 2 index iterating");
						break;
					}
				}
			}
		});
	}

	fn iter_pair_mut<'ext_env, Func>(
		&'ext_env mut self,
		func: &Func,
	) where
		Func: for<'scope> Fn(&'scope mut T, &'scope mut T, usize) + 'ext_env + Sync + Send,
		T: Send + Sync {
		self.values.for_each_mut(&|chunk,_|{
			chunk.iter_pair_mut(&|a,b,dim|{
				func(a,b,dim);
			});
		});
		self.values.iter_pair_mut(&|chunka: &mut NDimArray<Just<NDimIndexerU<DIM>>, DIM, T, Vec<T>>,chunkb,dim|{
			let a_uw=chunka.unwrap_mut();
			let b_uw=chunkb.unwrap_mut();
			let a_end_i=a_uw.0.lens()[dim] as isize-1;
			let mut a_idx_op=NDimIndexOperator::from_index(a_uw.0.deref().deref(), array::from_fn::<_,DIM,_>(|d|{
				if d==dim{a_end_i} else {0}
			})).unwrap();
			let mut b_idx_op = NDimIndexOperator::from_index(b_uw.0.deref().deref(), [0;DIM]).unwrap();
			
			loop {
				let a_item=unsafe {
					transmute(&mut a_uw.1[a_idx_op.get_compressed()])
				};
				let b_item=unsafe {
					transmute(&mut b_uw.1[b_idx_op.get_compressed()])
				};
				func(a_item,b_item,dim);
				// func(&mut a_uw.1[a_idx_op.get_compressed()],&mut b_uw.1[b_idx_op.get_compressed()],dim);
				let c=a_idx_op.move_n_carry(1);
				if c!=0 {break;}// iterating is over
				let mut dim_carry=false;

				if a_idx_op.get()[dim]==0 {// a idx change at mid
					a_idx_op.set_n_at_dim(dim, a_end_i);
					if cfg!(debug_assertions){
						dim_carry=true;
					}
				}
				let c=b_idx_op.move_n_carry(1);
				if c!=0 {// iterating is over
					debug_assert!(false,"not synchronous carry of 2 index iterating");
					break;
				}
				if b_idx_op.get()[dim]!=0 {// b idx change at mid
					b_idx_op.set_n_at_dim(dim, 0);
					if 1<=dim {
						let c=b_idx_op.move_n_carry_at_dim(dim-1, 1);
						if c!=0 {// iterating is over
							debug_assert_eq!(dim_carry,true,"not synchronous carry of 2 index iterating");
							break;
						}
					}else {// iterating is over
						debug_assert!(false,"not synchronous carry of 2 index iterating");
						break;
					}
				}
			}
		});
	}
}

impl<const DIM:usize,T> TNDimArrayParallelIterPair<DIM, T> for NDimChunkArray<DIM,T>{
	fn iter_pair_parallel<'ext_env,Func,TScopeCreator>(&'ext_env self,func:&Func,scope_creator:&TScopeCreator)
		where Func: for<'scope> Fn(&'scope T,&'scope T,usize)+'ext_env+Sync+Send,
		TScopeCreator: crate::traits::scope_no_ret::ThreadScopeCreator+Sync,
			T:Send+Sync 
	{
		self.values.for_each_parallel(&|chunk,_|{
			chunk.iter_pair(&|a,b,dim|{
				func(a,b,dim);
			});
		}, scope_creator);
		self.values.iter_pair_parallel(&|chunka: &NDimArray<Just<NDimIndexerU<DIM>>, DIM, T, Vec<T>>,chunkb,dim|{
			let a_uw=chunka.unwrap_ref();
			let b_uw=chunkb.unwrap_ref();
			let a_end_i=a_uw.0.lens()[dim] as isize-1;
			let mut a_idx_op=NDimIndexOperator::from_index(a_uw.0.deref(), array::from_fn::<_,DIM,_>(|d|{
				if d==dim{a_end_i} else {0}
			})).unwrap();
			let mut b_idx_op = NDimIndexOperator::from_index(b_uw.0.deref(), [0;DIM]).unwrap();
			
			loop {
				let a_item=unsafe {
					transmute(&a_uw.1[a_idx_op.get_compressed()])
				};
				let b_item=unsafe {
					transmute(&b_uw.1[b_idx_op.get_compressed()])
				};
				func(a_item,b_item,dim);
				// func(&mut a_uw.1[a_idx_op.get_compressed()],&mut b_uw.1[b_idx_op.get_compressed()],dim);
				let c=a_idx_op.move_n_carry(1);
				if c!=0 {break;}// iterating is over
				let mut dim_carry=false;

				if a_idx_op.get()[dim]==0 {// a idx change at mid
					a_idx_op.set_n_at_dim(dim, a_end_i);
					if cfg!(debug_assertions){
						dim_carry=true;
					}
				}
				let c=b_idx_op.move_n_carry(1);
				if c!=0 {// iterating is over
					debug_assert!(false,"not synchronous carry of 2 index iterating");
					break;
				}
				if b_idx_op.get()[dim]!=0 {// b idx change at mid
					b_idx_op.set_n_at_dim(dim, 0);
					if 1<=dim {
						let c=b_idx_op.move_n_carry_at_dim(dim-1, 1);
						if c!=0 {// iterating is over
							debug_assert_eq!(dim_carry,true,"not synchronous carry of 2 index iterating");
							break;
						}
					}else {// iterating is over
						debug_assert!(false,"not synchronous carry of 2 index iterating");
						break;
					}
				}
			}
		}, scope_creator);
	}
	fn iter_pair_mut_parallel<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:&Func,scope_creator:&TScopeCreator)
        where Func: for<'scope> Fn(&'scope mut T,&'scope mut T,usize)+'ext_env+Sync+Send,
        	TScopeCreator: crate::traits::scope_no_ret::ThreadScopeCreator+Sync,
            T:Send+Sync 
	{
		self.values.for_each_mut_parallel(&|chunk,_|{
			chunk.iter_pair_mut_parallel(&|a,b,dim|{
				func(a,b,dim);
			}, scope_creator);
		}, scope_creator);
		self.values.iter_pair_mut_parallel(&|chunka: &mut NDimArray<Just<NDimIndexerU<DIM>>, DIM, T, Vec<T>>,chunkb,dim|{
			let a_uw=chunka.unwrap_mut();
			let b_uw=chunkb.unwrap_mut();
			let a_end_i=a_uw.0.lens()[dim] as isize-1;
			let mut a_idx_op=NDimIndexOperator::from_index(a_uw.0.deref().deref(), array::from_fn::<_,DIM,_>(|d|{
				if d==dim{a_end_i} else {0}
			})).unwrap();
			let mut b_idx_op = NDimIndexOperator::from_index(b_uw.0.deref().deref(), [0;DIM]).unwrap();
			
			loop {
				let a_item=unsafe {
					transmute(&mut a_uw.1[a_idx_op.get_compressed()])
				};
				let b_item=unsafe {
					transmute(&mut b_uw.1[b_idx_op.get_compressed()])
				};
				func(a_item,b_item,dim);
				// func(&mut a_uw.1[a_idx_op.get_compressed()],&mut b_uw.1[b_idx_op.get_compressed()],dim);
				let c=a_idx_op.move_n_carry(1);
				if c!=0 {break;}// iterating is over
				let mut dim_carry=false;

				if a_idx_op.get()[dim]==0 {// a idx change at mid
					a_idx_op.set_n_at_dim(dim, a_end_i);
					if cfg!(debug_assertions){
						dim_carry=true;
					}
				}
				let c=b_idx_op.move_n_carry(1);
				if c!=0 {// iterating is over
					debug_assert!(false,"not synchronous carry of 2 index iterating");
					break;
				}
				if b_idx_op.get()[dim]!=0 {// b idx change at mid
					b_idx_op.set_n_at_dim(dim, 0);
					if 1<=dim {
						let c=b_idx_op.move_n_carry_at_dim(dim-1, 1);
						if c!=0 {// iterating is over
							debug_assert_eq!(dim_carry,true,"not synchronous carry of 2 index iterating");
							break;
						}
					}else {// iterating is over
						debug_assert!(false,"not synchronous carry of 2 index iterating");
						break;
					}
				}
			}
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
	use std::sync::{Arc, Mutex};

use crate::{structures::n_dim_array::t_n_dim_indexer::TNDimIndexer, traits::scope_no_ret::ThreadScopeCreatorStd};

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
	#[test]
	fn test_par_iter(){
		let ranges=[10,15,7];
		let chunk_array=NDimChunkArray::<3,NDimIndex<3>>::from_fn(ranges,4,|idx|{
			*idx
		});
		let count=Arc::new(Mutex::new(0));
		let exp_count=ranges.iter().fold(1, |a,b|a*b);
		chunk_array.for_each_parallel(&|v,idx|{
			assert_eq!(*v,idx);
			// print!("{:?}",idx);
			*(count.lock().unwrap())+=1;
		}, &ThreadScopeCreatorStd);
		assert_eq!(*count.lock().unwrap(),exp_count);
	}

	#[test]
	fn test_shape(){
		const DIM:usize=3;
		let ranges=[3,3,3];
		let chunk_array=NDimChunkArray::from_fn(ranges,2,|idx|{
			(*idx,array::from_fn::<Option<NDimIndex<DIM>>,DIM,_>(|_|None))
		});
		for a in chunk_array.values.n_dim_index().iter() {
			println!("{:?}'s shape: {:?}",a,chunk_array.values.get(&a).unwrap().n_dim_index().lens());
		}
	}

	#[test]
	fn test_par_iter_pair(){
		const DIM:usize=3;
		let ranges=[3,3,3];
		let mut chunk_array=NDimChunkArray::from_fn(ranges,2,|idx|{
			(*idx,array::from_fn::<Option<NDimIndex<DIM>>,DIM,_>(|_|None))
		});
		// let count=Arc::new(Mutex::new(0));
		// let exp_count=ranges.iter().fold(1, |a,b|a*(b-1))*DIM;
		chunk_array.iter_pair_mut_parallel(&|a,b,dim_idx|{
			println!("({:?},{:?}) at {}",a.0,b.0,dim_idx);
			a.1[dim_idx]=Some(b.0.clone());
			// *count.lock().unwrap()+=1;
		}, &ThreadScopeCreatorStd);
		// assert_eq!(*count.lock().unwrap(),exp_count);
		for a in chunk_array.iter() {
			println!("{:?}",a);
		}
	}

	#[test]
	fn test_par_iter_pair_edge(){
		const DIM:usize=3;
		let ranges=[3,3,3];
		let mut chunk_array=NDimChunkArray::from_fn(ranges,1,|idx|{
			(*idx,array::from_fn::<Option<NDimIndex<DIM>>,DIM,_>(|_|None))
		});
		// let count=Arc::new(Mutex::new(0));
		// let exp_count=ranges.iter().fold(1, |a,b|a*(b-1))*DIM;
		chunk_array.iter_pair_mut_parallel(&|a,b,dim_idx|{
			println!("({:?},{:?}) at {}",a.0,b.0,dim_idx);
			a.1[dim_idx]=Some(b.0.clone());
			// *count.lock().unwrap()+=1;
		}, &ThreadScopeCreatorStd);
		// assert_eq!(*count.lock().unwrap(),exp_count);
		for a in chunk_array.iter() {
			println!("{:?}",a);
		}
	}
}