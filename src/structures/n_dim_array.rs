use std::{array, mem::transmute, ops::{Deref, Index, IndexMut, Range}};

use crate::{structures::n_dim_index::{NDimIndex, NDimIndexer, TNDimIndexer}, traits::scope_no_ret::{self, ThreadScopeCreator, ThreadScopeCreatorStd, ThreadScopeUser}};

#[derive(Debug)]
pub struct NDimArray<TIndexer,const DIM:usize,T,Storage>
    where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
    Storage:Index<usize,Output=T>+IndexMut<usize>
{
    values:Storage,
    n_dim_index:TIndexer
}

impl<TIndexer,const DIM:usize,T> NDimArray<TIndexer,DIM,T,Vec<T>>  
    where TIndexer:Deref<Target : TNDimIndexer<DIM>>
{
    pub fn from_fn<Fn>(n_dim_index:TIndexer,mut func:Fn)->Self
        where Fn:FnMut(NDimIndex<DIM>)->T
    {
        let index_iter=n_dim_index.iter();
        let values=index_iter.map(|index|func(index)).collect();
        Self { values, n_dim_index }
    }
}

pub trait TNDimArray<const DIM:usize,T>
{

	fn lens(&self)->impl Deref<Target=[Range<isize>;DIM]>;

    fn get(&self,indexes:&NDimIndex<DIM>)->Option<&T>;
    fn get_mut(&mut self,indexes:&NDimIndex<DIM>)->Option<&mut T>;

	fn parallel_iter<'ext_env,Func,TScopeCreator>(&'ext_env self,func:&Func,scope_creator:&TScopeCreator)
		where Func: for<'scope> Fn(&'scope T,&'scope NDimIndex<DIM>)+'ext_env+Sync+Send,
		TScopeCreator: ThreadScopeCreator+Sync,
		T:Send+Sync;
		
	fn parallel_iter_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:&Func,scope_creator:&TScopeCreator)
		where Func: for<'scope> Fn(&'scope mut T,&'scope NDimIndex<DIM>)+'ext_env+Sync+Send,
		TScopeCreator: ThreadScopeCreator+Sync,
		T:Send+Sync;
}

pub trait TNDimArrayGetWithNeiborhoods<const DIM:usize,T> : TNDimArray<DIM,T>  {
	fn get_with_neiborhoods_generic<'a,ForNeiborhood,NeiborhoodResult>(&'a self,index:&NDimIndex<DIM>,for_neiborhood:ForNeiborhood)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,NeiborhoodResult>>
		where ForNeiborhood:Fn(&'a Self,&mut NDimIndex<DIM>,usize,bool)->NeiborhoodResult;

	fn get_mut_with_neiborhoods_generic<'a,ForNeiborhood,NeiborhoodResult>(&'a mut self,index:&NDimIndex<DIM>,for_neiborhood:ForNeiborhood)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,NeiborhoodResult>>
		where ForNeiborhood:FnMut(&mut Self,&mut NDimIndex<DIM>,usize,bool)->NeiborhoodResult;

	
	fn get_with_neiborhoods<'a>(&'a self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,Option<&'a T>>>{
		self.get_with_neiborhoods_generic(index,|s,i,_,_|s.get(i))
	}

	fn get_mut_with_neiborhoods_mut<'a>(&'a mut self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a mut T>>>{
		self.get_mut_with_neiborhoods_generic(index,|s,i,_,_|unsafe{transmute(s.get_mut(i))})
	}

	fn get_mut_with_neiborhoods<'a>(&'a mut self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a T>>>{
		self.get_mut_with_neiborhoods_generic(index,|s,i,_,_|unsafe{transmute(s.get(i))})
	}

	fn get_with_neiborhoods_loop<'a>(&'a self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,Option<&'a T>>>{
		self.get_with_neiborhoods_generic(index,|s,i,dim,dir|{
			let range=&self.lens()[dim];
			if dir {
				if i[dim]>=range.end {
					i[dim]-=range.end - range.start;
				}
			}else {
				if i[dim]<range.start {
					i[dim]+=range.end - range.start;
				}
			}
			s.get(i)
		})
	}

	fn get_mut_with_neiborhoods_loop<'a>(&'a mut self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a T>>>{
		self.get_mut_with_neiborhoods_generic(index,|s,i,dim,dir|{
			let range=&s.lens()[dim];
			if range.end-range.start<=1 {
				return None;
			}
			if dir {
				if i[dim]>=range.end {
					i[dim]-=range.end - range.start;
				}
			}else {
				if i[dim]<range.start {
					i[dim]+=range.end - range.start;
				}
			}
			unsafe{transmute(s.get(i))}
		})
	}
}

pub trait TNDimArrayParallelIterPair<const DIM:usize,T> : TNDimArray<DIM,T> {
    fn parallel_iter_pair<'ext_env,Func,TScopeCreator>(&'ext_env self,func:&Func,scope_creator:&TScopeCreator)
        where Func: for<'scope> Fn(&'scope T,&'scope T,usize)+'ext_env+Sync+Send,
        TScopeCreator: ThreadScopeCreator+Sync,
		T:Send+Sync;
    
    fn parallel_iter_pair_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:&Func,scope_creator:&TScopeCreator)
        where Func: for<'scope> Fn(&'scope mut T,&'scope mut T,usize)+'ext_env+Sync+Send,
        TScopeCreator: ThreadScopeCreator+Sync,
		T:Send+Sync;
}

impl<TIndexer,const DIM:usize,T,Storage> NDimArray<TIndexer,DIM,T,Storage>  
	where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
	Storage:Index<usize,Output=T>+IndexMut<usize>
{
	pub fn new(n_dim_index:TIndexer,values:Storage)->Self{Self{values,n_dim_index}}

	pub fn n_dim_index(&self)->&TIndexer{&self.n_dim_index}
	
	pub fn get_with_compressed(&self,index:usize)->Option<&T> {
		if !self.n_dim_index.contains_compressed(index){return None;}
		Some(self.values.index(index))
	}
	pub fn get_mut_with_compressed(&mut self,index:usize)->Option<&mut T> {
		if !self.n_dim_index.contains_compressed(index){return None;}
		Some(self.values.index_mut(index))
	}
	pub fn values(&self)->&Storage {
		&self.values
	}
	pub unsafe fn get_other<'a,'b>(&'a self,indexes:&NDimIndex<DIM>)->Option<&'b T>
	{
		self.get(indexes).map(|a|unsafe{  &*(a as *const T)})
	}
	pub unsafe fn get_mut_other<'a,'b>(&'a mut self,indexes:&NDimIndex<DIM>)->Option<&'b mut T>
	{
		self.get_mut(indexes).map(|a|unsafe{  &mut *(a as *mut T)})
	}

	
}

impl<TIndexer,const DIM:usize,T,Storage> TNDimArray<DIM, T> for NDimArray<TIndexer,DIM,T,Storage>  
	where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
	Storage:Index<usize,Output=T>+IndexMut<usize>
{
	fn lens(&self)->impl Deref<Target = [std::ops::Range<isize>; DIM]> {
		self.n_dim_index.lens()
	}

	fn get(&self,indexes:&NDimIndex<DIM>)->Option<&T>{
		if self.n_dim_index.contains(indexes){
			Some(self.values.index(self.n_dim_index.compress_index(indexes)))
		}else {
			None
		}
	}
	fn get_mut(&mut self,indexes:&NDimIndex<DIM>)->Option<&mut T>{
		if !self.n_dim_index.contains(indexes){return None;}
		Some(self.values.index_mut(self.n_dim_index.compress_index(indexes)))
	}
	
	fn parallel_iter<'ext_env,Func,TScopeCreator>(&'ext_env self,func:&Func,scope_creator:&TScopeCreator)
		where Func: for<'scope> Fn(&'scope T,&'scope NDimIndex<DIM>)+'ext_env+Sync+Send,
			TScopeCreator: ThreadScopeCreator+Sync,
			T:Send+Sync
	{
		struct A<'env,Func,TIndexer,const DIM:usize,T,Storage>
			where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
			Storage:Index<usize,Output=T>+IndexMut<usize>
		{
			array:&'env NDimArray<TIndexer,DIM,T,Storage>,
			func:&'env Func,
		}
		impl<'env,Func,TIndexer,const DIM:usize,T,Storage> ThreadScopeUser<'env> for A<'env,Func,TIndexer,DIM,T,Storage>
			where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
			Storage:Index<usize,Output=T>+IndexMut<usize>,
			Func: for<'scope> Fn(&'scope T,&'scope NDimIndex<DIM>)+'env+Sync+Send,
			T:Send+Sync 
		{
			fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->()
				where TScope:'scope+scope_no_ret::ThreadScope<'scope>,
					'env:'scope 
			{
				let array=self.array;
				let func=self.func;
				for index in array.n_dim_index.iter().enumerate() {
					if let Some(item)=array.get_with_compressed(index.0) {
						scope.spawn( move||{
							(func)(item,&index.1);
						});
					}
				}
			}
		}
		scope_creator.scope(
			A{array:self,func:&func}
		);
	}
	
	fn parallel_iter_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:&Func,scope_creator:&TScopeCreator)
		where Func: for<'scope> Fn(&'scope mut T,&'scope NDimIndex<DIM>)+'ext_env+Sync+Send,
			TScopeCreator: ThreadScopeCreator+Sync,
			T:Send+Sync 
	{
		struct A<'env,Func,TIndexer,const DIM:usize,T,Storage>
			where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
			Storage:Index<usize,Output=T>+IndexMut<usize>
		{
			array:&'env mut NDimArray<TIndexer,DIM,T,Storage>,
			func:&'env Func,
		}
		impl<'env,Func,TIndexer,const DIM:usize,T,Storage> ThreadScopeUser<'env> for A<'env,Func,TIndexer,DIM,T,Storage>
			where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
			Storage:Index<usize,Output=T>+IndexMut<usize>,
			Func: for<'scope> Fn(&'scope mut T,&'scope NDimIndex<DIM>)+'env+Sync+Send,
			T:Send+Sync 
		{
			fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->()
				where TScope:'scope+scope_no_ret::ThreadScope<'scope>,
					'env:'scope 
			{
				let array=&mut *self.array;
				let func=self.func;
				let iter:&TIndexer=unsafe {
					transmute(&array.n_dim_index)
				};
				// let iter:&TIndexer=&array.n_dim_index;
				for index in iter.iter().enumerate() {
					if let Some(item)=unsafe{transmute(array.get_mut_with_compressed(index.0))} {
						scope.spawn( move||{
							(func)(item,&index.1);
						});
					}
				}
			}
		}
		scope_creator.scope(
			A{array:self,func:&func}
		);
	}
	
}

impl<TIndexer,const DIM:usize,T,Storage> TNDimArrayGetWithNeiborhoods<DIM, T> for NDimArray<TIndexer,DIM,T,Storage>  
	where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
	Storage:Index<usize,Output=T>+IndexMut<usize>{
	fn get_with_neiborhoods_generic<'a,ForNeiborhood,NeiborhoodResult>(&'a self,index:&NDimIndex<DIM>,for_neiborhood:ForNeiborhood)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,NeiborhoodResult>>
		where ForNeiborhood:Fn(&'a Self,&mut NDimIndex<DIM>,usize,bool)->NeiborhoodResult
	{
		self.get(index).map(|res_cur|{
			let res_neiborhoods = array::from_fn(|dim|{
				let mut nindex_0=index.clone();
				nindex_0[dim]+=1;
				let res_0=for_neiborhood(self,&mut nindex_0,dim,true);
				let mut nindex_1=index.clone();
				nindex_1[dim]-=1;
				let res_1=for_neiborhood(self,&mut nindex_1,dim,false);
				return ((res_0,nindex_0),(res_1,nindex_1));

			});
			NDimArrayGetWithNeiborhoodsResult { cur: res_cur, neiborhoods: res_neiborhoods }
		})
	}

	fn get_mut_with_neiborhoods_generic<'a,ForNeiborhood,NeiborhoodResult>(&'a mut self,index:&NDimIndex<DIM>,mut for_neiborhood:ForNeiborhood)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,NeiborhoodResult>>
		where ForNeiborhood:FnMut(&mut Self,&mut NDimIndex<DIM>,usize,bool)->NeiborhoodResult
	{
		let res_cur_may=unsafe {
			self.get_mut_other(index)
		};

		if let Some(res_cur)=res_cur_may{

			let res_neiborhoods = array::from_fn(|dim|{
				let mut nindex_0=index.clone();
				nindex_0[dim]+=1;
				let res_0=for_neiborhood(self,&mut nindex_0,dim,true);
				let mut nindex_1=index.clone();
				nindex_1[dim]-=1;
				let res_1=for_neiborhood(self,&mut nindex_1,dim,false);
				return ((res_0,nindex_0),(res_1,nindex_1));

			});
			return Some(NDimArrayGetWithNeiborhoodsResult { cur: res_cur, neiborhoods: res_neiborhoods });
		}else {
			return None;
		}
	}

}

impl<TIndexer,const DIM:usize,T,Storage> TNDimArrayParallelIterPair<DIM, T> for NDimArray<TIndexer,DIM,T,Storage>  
	where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
	Storage:Index<usize,Output=T>+IndexMut<usize>  {
	fn parallel_iter_pair<'ext_env,Func,TScopeCreator>(&'ext_env self,func:&Func,scope_creator:&TScopeCreator)
		where Func: for<'scope> Fn(&'scope T,&'scope T,usize)+'ext_env+Sync+Send,
		TScopeCreator: ThreadScopeCreator+Sync,
			T:Send+Sync {
		for mut_dim in 0..DIM {
			struct AScopeUser<'env,Func,TIndexer,const DIM:usize,T,Storage>
				where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
				Storage:Index<usize,Output=T>+IndexMut<usize>
			{
				values:&'env NDimArray<TIndexer,DIM,T,Storage>,
				mut_dim:usize,
				func:&'env Func,
				plus_1:bool,
			}
			impl<'env,Func,TIndexer,const DIM:usize,T,Storage> ThreadScopeUser<'env> for AScopeUser<'env,Func,TIndexer,DIM,T,Storage> 
				where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
				Storage:Index<usize,Output=T>+IndexMut<usize>,
				Func: for<'scope> Fn(&'scope T,&'scope T,usize)+'env+Sync+Send,
				T:Send+Sync,
				
			{
				fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->()
					where TScope:'scope+scope_no_ret::ThreadScope<'scope>,
						'env:'scope {
					let rem=if self.plus_1 {0} else {1};
					let values=self.values;
					let n_dim_index:&TIndexer=unsafe {
						transmute(&values.n_dim_index)
					};
					let mut_dim=self.mut_dim;
					let func=self.func;
					for p in n_dim_index.iter() {
						if p[mut_dim]%2==rem {
							continue;
						}
						let mut p2=p.clone();
						p2[mut_dim]+=1;
						if let Some(v1)=unsafe{values.get_other(&p)}{
							if let Some(v2)=unsafe{values.get_other(&p2)}{
								scope.spawn( move||{
									(func)(v1,v2,mut_dim);
								});
							}
						}
					}
				}
			}
			scope_creator.scope(
				AScopeUser{values:self,mut_dim,func:&func,plus_1:false}
			);
			scope_creator.scope(
				AScopeUser{values:self,mut_dim,func:&func,plus_1:true}
			);
		}
	}
	fn parallel_iter_pair_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:&Func,scope_creator:&TScopeCreator)
		where Func: for<'scope> Fn(&'scope mut T,&'scope mut T,usize)+'ext_env+Sync+Send,
		TScopeCreator: ThreadScopeCreator+Sync,
		T:Send+Sync,
	{
		for mut_dim in 0..DIM {
			struct AScopeUser<'env,Func,TIndexer,const DIM:usize,T,Storage>
				where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
				Storage:Index<usize,Output=T>+IndexMut<usize>
			{
				values:&'env mut NDimArray<TIndexer,DIM,T,Storage>,
				mut_dim:usize,
				func:&'env Func,
				plus_1:bool,
			}
			impl<'env,Func,TIndexer,const DIM:usize,T,Storage> ThreadScopeUser<'env> for AScopeUser<'env,Func,TIndexer,DIM,T,Storage> 
				where TIndexer:Deref<Target : TNDimIndexer<DIM>>,
				Storage:Index<usize,Output=T>+IndexMut<usize>,
				Func: for<'scope> Fn(&'scope mut T,&'scope mut T,usize)+'env+Sync+Send,
				T:Send+Sync,
				
			{
				fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->()
					where TScope:'scope+scope_no_ret::ThreadScope<'scope>,
						'env:'scope {
					let rem=if self.plus_1 {0} else {1};
					let values=self.values;
					let n_dim_index:&TIndexer=unsafe {
						transmute(&values.n_dim_index)
					};
					let mut_dim=self.mut_dim;
					let func=self.func;
					for p in n_dim_index.iter() {
						if p[mut_dim]%2==rem {
							continue;
						}
						let mut p2=p.clone();
						p2[mut_dim]+=1;
						if let Some(v1)=unsafe{values.get_mut_other(&p)}{
							if let Some(v2)=unsafe{values.get_mut_other(&p2)}{
								scope.spawn( move||{
									(func)(v1,v2,mut_dim);
								});
							}
						}
					}
				}
			}
			scope_creator.scope(
				AScopeUser{values:self,mut_dim,func:&func,plus_1:false}
			);
			scope_creator.scope(
				AScopeUser{values:self,mut_dim,func:&func,plus_1:true}
			);
		}
	}
}

#[derive(Debug)]
pub struct NDimArrayGetWithNeiborhoodsResult<const DIM:usize,T,TNeiborhood>{
    pub cur:T,
    pub neiborhoods:[ ((TNeiborhood,NDimIndex<DIM>),(TNeiborhood,NDimIndex<DIM>)) ;DIM]
}
// #[derive(Debug)]
// pub struct NDimArrayGetMutWithNeiborhoodsResult<'a,const DIM:usize,T>{
//     pub cur:&'a mut T,
//     pub neiborhoods:[ ((Option<&'a T>,NDimIndex<DIM>),(Option<&'a T>,NDimIndex<DIM>)) ;DIM]
// }
// #[derive(Debug)]
// pub struct NDimArrayGetMutWithNeiborhoodsMutResult<'a,const DIM:usize,T>{
//     pub cur:&'a mut T,
//     pub neiborhoods:[ ((Option<&'a mut T>,NDimIndex<DIM>),(Option<&'a mut T>,NDimIndex<DIM>)) ;DIM]
// }



#[cfg(test)]
mod test{

    // use core::time;
    // use std::thread;

    // use crate::structures::{n_dim_array::{NDimArray}, n_dim_index::NDimIndexer};
	use super::*;

    #[test]
    fn test_ndim_array_bi_iter_mut(){
        // const ADIM:usize=3;
        let andidx=NDimIndexer::new_len([0..3,0..3,0..3]);
        let mut andarr=NDimArray::from_fn(&andidx, |idx|(idx,Vec::<NDimIndex<3>>::new()));
		for i in andidx.iter() {
			println!("{:?} : {:?}", i, andarr.get(&i).unwrap().0);
		}
		andarr.parallel_iter_pair_mut(&|a,b,_|{
			// thread::sleep(time::Duration::from_millis());
			println!("({:?},{:?})",a.0,b.0);
			a.1.push(b.0.clone());
			b.1.push(a.0.clone());
		}, &ThreadScopeCreatorStd);
        // for awdawd in andidx.iter() {
        //     let got=andarr.get_mut_with_neiborhoods(&awdawd);
        //     println!("{:#?}",got);
        // }
		for i in andidx.iter() {
			println!("{:?} : {:?}", i, andarr.get(&i).unwrap().1);
		}
		
    }
}