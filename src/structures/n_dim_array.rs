use std::{array, mem::transmute, ops::{Deref, Index, IndexMut}};

use crate::{structures::n_dim_index::{NDimIndex, NDimIndexer}, traits::scope::{self, ThreadScopeCreator, ThreadScopeCreatorStd, ThreadScopeUser}};

#[derive(Debug)]
pub struct NDimArray<TIndexerIter,const DIM:usize,T,Storage>
    where TIndexerIter:Deref<Target = NDimIndexer<DIM>>,
    Storage:Index<usize,Output=T>+IndexMut<usize>
{
    values:Storage,
    n_dim_index:TIndexerIter
}

impl<TIndexerIter,const DIM:usize,T> NDimArray<TIndexerIter,DIM,T,Vec<T>>  
    where TIndexerIter:Deref<Target = NDimIndexer<DIM>>
{
    pub fn from_fn<Fn>(n_dim_index:TIndexerIter,mut func:Fn)->Self
        where Fn:FnMut(NDimIndex<DIM>)->T
    {
        let index_iter=n_dim_index.iter();
        let values=index_iter.map(|index|func(index)).collect();
        Self { values, n_dim_index }
    }
}

impl<TIndexerIter,const DIM:usize,T,Storage> NDimArray<TIndexerIter,DIM,T,Storage>  
    where TIndexerIter:Deref<Target = NDimIndexer<DIM>>,
    Storage:Index<usize,Output=T>+IndexMut<usize>

{

    pub fn new(n_dim_index:TIndexerIter,values:Storage)->Self{Self{values,n_dim_index}}

    pub fn values(&self)->&Storage {
        &self.values
    }

    pub fn n_dim_index(&self)->&NDimIndexer<DIM>{&self.n_dim_index}

    pub fn get(&self,indexes:&NDimIndex<DIM>)->Option<&T>{
        if self.n_dim_index.contains(indexes){
            Some(self.values.index(self.n_dim_index.compress_index_u(indexes)))
        }else {
            None
        }
    }
    pub fn get_mut(&mut self,indexes:&NDimIndex<DIM>)->Option<&mut T>{
        if !self.n_dim_index.contains(indexes){return None;}
        Some(self.values.index_mut(self.n_dim_index.compress_index_u(indexes)))
    }

    pub fn get_with_compressed(&self,index:usize)->Option<&T> {
        if !self.n_dim_index.contains_compressed_u(index){return None;}
        Some(self.values.index(index))
    }
    pub fn get_mut_with_compressed(&mut self,index:usize)->Option<&mut T> {
        if !self.n_dim_index.contains_compressed_u(index){return None;}
        Some(self.values.index_mut(index))
    }


	pub unsafe fn get_other<'a,'b>(&'a self,indexes:&NDimIndex<DIM>)->Option<&'b T>
	{
		self.get(indexes).map(|a|unsafe{  &*(a as *const T)})
	}
	pub unsafe fn get_mut_other<'a,'b>(&'a mut self,indexes:&NDimIndex<DIM>)->Option<&'b mut T>
	{
		self.get_mut(indexes).map(|a|unsafe{  &mut *(a as *mut T)})
	}

	//  pub fn get_mut_ptr(&self,indexes:NDimIndex<DIM>)->Option<*mut T>{
	// 	if !self.n_dim_index.contains(indexes){return None;}
	// 	self.get(indexes).map(|a|a as *mut T)
	// 	//self.values.pt
	// }
    // pub fn get_ptr(&self,indexes:&NDimIndex<DIM>)->Option<*const T>{
    //     if !self.n_dim_index.contains(indexes){return None;}
    //     self.get(indexes).map(|a|a as *const T)
    //     //self.values.pt
    // }
	// pub fn get_mut_unsafe(&self,indexes:NDimIndex<DIM>)->Option<&mut T>{
	// 	if !self.n_dim_index.contains(indexes){return None;}
    //     self.get_mut_ptr(indexes).map(|a|unsafe {&mut *a})
	// }
    // pub fn get_mut_ptr(&self,indexes:NDimIndex<DIM>)->Option<*mut T>{
    //     if !self.n_dim_index.contains(indexes){return None;}
    //     self.get(indexes).map(|a|a as *mut T)
    //     //self.values.pt
    // }

	pub fn get_with_neiborhoods_generic<'a,ForNeiborhood,NeiborhoodResult>(&'a self,index:&NDimIndex<DIM>,for_neiborhood:ForNeiborhood)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,NeiborhoodResult>>
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

	pub fn get_mut_with_neiborhoods_generic<'a,ForNeiborhood,NeiborhoodResult>(&'a mut self,index:&NDimIndex<DIM>,mut for_neiborhood:ForNeiborhood)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,NeiborhoodResult>>
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
		// unsafe {
		// 	self.get_mut_other(index).map(|res_cur|{
		// 		let res_neiborhoods = array::from_fn(|dim|{
		// 			let mut nindex_0=index.clone();
		// 			nindex_0[dim]+=1;
		// 			let res_0=for_neiborhood(self,&mut nindex_0);
		// 			let mut nindex_1=index.clone();
		// 			nindex_1[dim]-=1;
		// 			let res_1=for_neiborhood(self,&mut nindex_1);
		// 			return ((res_0,nindex_0),(res_1,nindex_1));
	
		// 		});
		// 		NDimArrayGetWithNeiborhoodsResult { cur: res_cur, neiborhoods: res_neiborhoods }
		// 	})
		// }
	}

	pub fn get_with_neiborhoods<'a>(&'a self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,Option<&'a T>>>{
		self.get_with_neiborhoods_generic(index,|s,i,_,_|s.get(i))
	}

	pub fn get_mut_with_neiborhoods_mut<'a>(&'a mut self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a mut T>>>{
		self.get_mut_with_neiborhoods_generic(index,|s,i,_,_|unsafe{s.get_mut_other(i)})
	}

	pub fn get_mut_with_neiborhoods<'a>(&'a mut self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a T>>>{
		self.get_mut_with_neiborhoods_generic(index,|s,i,_,_|unsafe{s.get_other(i)})
	}

	pub fn get_with_neiborhoods_loop<'a>(&'a self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,Option<&'a T>>>{
		self.get_with_neiborhoods_generic(index,|s,i,dim,dir|{
			let range=&s.n_dim_index.lens()[dim];
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

	pub fn get_mut_with_neiborhoods_loop<'a>(&'a mut self,index:&NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a T>>>{
		self.get_mut_with_neiborhoods_generic(index,|s,i,dim,dir|{
			let range=&s.n_dim_index.lens()[dim];
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
			unsafe{s.get_other(i)}
		})
	}

    
    pub fn parallel_iter_pair_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:Func,mut scope_creator:TScopeCreator)
        where Func: for<'scope> Fn(&'scope mut T,&'scope mut T,usize)+'ext_env+Sync+Send,
        TScopeCreator: ThreadScopeCreator<(),()>,
		T:Send+Sync,
    {
        for mut_dim in 0..DIM {

            struct AScopeUser<'env,Func,TIndexerIter,const DIM:usize,T,Storage>
                where TIndexerIter:Deref<Target = NDimIndexer<DIM>>,
                Storage:Index<usize,Output=T>+IndexMut<usize>
            {
                values:&'env mut NDimArray<TIndexerIter,DIM,T,Storage>,
                mut_dim:usize,
                func:&'env Func,
                plus_1:bool,
            }
            impl<'env,Func,TIndexerIter,const DIM:usize,T,Storage> ThreadScopeUser<'env> for AScopeUser<'env,Func,TIndexerIter,DIM,T,Storage> 
                where TIndexerIter:Deref<Target = NDimIndexer<DIM>>,
                Storage:Index<usize,Output=T>+IndexMut<usize>,
				Func: for<'scope> Fn(&'scope mut T,&'scope mut T,usize)+'env+Sync+Send,
				T:Send+Sync,
				
            {
                type Output=();
            
                fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->Self::Output
                    where TScope:'scope+scope::ThreadScope<'scope,()>,
                        'env:'scope {
					let rem=if self.plus_1 {0} else {1};
					let values=self.values;
					let n_dim_index:&TIndexerIter=unsafe {
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
								// let f=&mut self.func;
								scope.spawn( move||{
									(func)(v1,v2,mut_dim);
								});
							}
						}
					}
                }
                
                type ScopeFnOutput=();
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

    use crate::structures::{n_dim_array::{NDimArray}, n_dim_index::NDimIndexer};

    #[test]
    fn test_ndim_array_bi_iter_mut(){
        // const ADIM:usize=3;
        let andidx=NDimIndexer::new_len([0..3,0..3,0..3]);
        let mut andarr=NDimArray::from_fn(&andidx, |idx|idx);
        for awdawd in andidx.iter() {
            let got=andarr.get_mut_with_neiborhoods(&awdawd);
            println!("{:#?}",got);
        }
    }
}