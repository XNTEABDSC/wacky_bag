use std::{array, ops::{Deref, Index, IndexMut}};

use crate::{structures::n_dim_index::{NDimIndex, NDimIndexer}, traits::scope::{self, ScopeCreator, ScopeUser}};

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

    pub fn get(&self,indexes:NDimIndex<DIM>)->Option<&T>{
        if self.n_dim_index.contains(indexes){
            Some(self.values.index(self.n_dim_index.compress_index_u(indexes)))
        }else {
            None
        }
    }
    pub fn get_mut(&mut self,indexes:NDimIndex<DIM>)->Option<&mut T>{
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


    pub fn get_ptr(&self,indexes:NDimIndex<DIM>)->Option<*const T>{
        if !self.n_dim_index.contains(indexes){return None;}
        self.get(indexes).map(|a|a as *const T)
        //self.values.pt
    }
    pub fn get_mut_ptr(&mut self,indexes:NDimIndex<DIM>)->Option<*mut T>{
        if !self.n_dim_index.contains(indexes){return None;}
        self.get_mut(indexes).map(|a|a as *mut T)
        //self.values.pt
    }
    pub fn get_with_neiborhoods<'a>(&'a self,index:NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,Option<&'a T>>>{
        let res_cur_ptr_may=self.get_ptr(index);

        if let Some(res_cur_ptr)=res_cur_ptr_may{

            let res_cur=unsafe {
                &*res_cur_ptr
            };

            let res_neiborhoods = array::from_fn(|dim|{
                let mut nindex_0=index.clone();
                nindex_0[dim]+=1;
                let res_0=self.get_ptr(nindex_0).map(|ptr|{
                    unsafe {
                        &*ptr
                    }
                });
                let mut nindex_1=index.clone();
                nindex_1[dim]-=1;
                let res_1=self.get_ptr(nindex_1).map(|ptr|{
                    unsafe {
                        &*ptr
                    }
                });
                return ((res_0,nindex_0),(res_1,nindex_1));

            });
            return Some(NDimArrayGetWithNeiborhoodsResult { cur: res_cur, neiborhoods: res_neiborhoods });
        }else {
            return None;
        }
        

    }

    pub fn get_with_neiborhoods_loop<'a>(&'a self,index:NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,Option<&'a T>>>{
        let res_cur_ptr_may=self.get_ptr(index);

        if let Some(res_cur_ptr)=res_cur_ptr_may{

            let res_cur=unsafe {
                &*res_cur_ptr
            };

            let res_neiborhoods = array::from_fn(|dim|{
                let dim_range=self.n_dim_index.lens()[dim].clone();
                let mut nindex_0=index.clone();
                nindex_0[dim]+=1;
                if dim_range.end<=nindex_0[dim]{
                    nindex_0[dim]=dim_range.start
                }
                let res_0=self.get_ptr(nindex_0).map(|ptr|{
                    unsafe {
                        &*ptr
                    }
                });
                let mut nindex_1=index.clone();
                nindex_1[dim]-=1;
                if nindex_0[dim]<dim_range.start{
                    nindex_0[dim]=dim_range.end-1;
                }
                let res_1=self.get_ptr(nindex_1).map(|ptr|{
                    unsafe {
                        &*ptr
                    }
                });
                return ((res_0,nindex_0),(res_1,nindex_1));

            });
            return Some(NDimArrayGetWithNeiborhoodsResult { cur: res_cur, neiborhoods: res_neiborhoods });
        }else {
            return None;
        }
        

    }


    pub fn get_mut_with_neiborhoods<'a>(&'a mut self,index:NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a T>>>{
        let res_cur_ptr_may=self.get_mut_ptr(index);

        if let Some(res_cur_ptr)=res_cur_ptr_may{

            let res_cur=unsafe {
                &mut *res_cur_ptr
            };

            let res_neiborhoods = array::from_fn(|dim|{
                let mut nindex_0=index.clone();
                nindex_0[dim]+=1;
                let res_0=self.get_ptr(nindex_0).map(|ptr|{
                    unsafe {
                        &*ptr
                    }
                });
                let mut nindex_1=index.clone();
                nindex_1[dim]-=1;
                let res_1=self.get_ptr(nindex_1).map(|ptr|{
                    unsafe {
                        &*ptr
                    }
                });
                return ((res_0,nindex_0),(res_1,nindex_1));

            });
            return Some(NDimArrayGetMutWithNeiborhoodsResult { cur: res_cur, neiborhoods: res_neiborhoods });
        }else {
            return None;
        }
        

    }

    pub fn get_mut_with_neiborhoods_loop<'a>(&'a mut self,index:NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a T>>>{
        let res_cur_ptr_may=self.get_mut_ptr(index);

        if let Some(res_cur_ptr)=res_cur_ptr_may{

            let res_cur=unsafe {
                &mut *res_cur_ptr
            };

            let res_neiborhoods = array::from_fn(|dim|{
                let dim_range=self.n_dim_index.lens()[dim].clone();
                let mut nindex_0=index.clone();
                nindex_0[dim]+=1;
                if dim_range.end<=nindex_0[dim]{
                    nindex_0[dim]=dim_range.start
                }
                let res_0=self.get_ptr(nindex_0).map(|ptr|{
                    unsafe {
                        &*ptr
                    }
                });
                let mut nindex_1=index.clone();
                nindex_1[dim]-=1;
                if nindex_0[dim]<dim_range.start{
                    nindex_0[dim]=dim_range.end-1;
                }
                let res_1=self.get_ptr(nindex_1).map(|ptr|{
                    unsafe {
                        &*ptr
                    }
                });
                return ((res_0,nindex_0),(res_1,nindex_1));

            });
            return Some(NDimArrayGetMutWithNeiborhoodsResult { cur: res_cur, neiborhoods: res_neiborhoods });
        }else {
            return None;
        }
        

    }

    
    pub fn get_mut_with_neiborhoods_mut<'a>(&'a mut self,index:NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a mut T>>>{
        let res_cur_ptr_may=self.get_mut_ptr(index);

        if let Some(res_cur_ptr)=res_cur_ptr_may{

            let res_cur=unsafe {
                &mut *res_cur_ptr
            };

            let res_neiborhoods = array::from_fn(|dim|{
                let mut nindex_0=index.clone();
                nindex_0[dim]+=1;
                let res_0=self.get_mut_ptr(nindex_0).map(|ptr|{
                    unsafe {
                        &mut *ptr
                    }
                });
                let mut nindex_1=index.clone();
                nindex_1[dim]-=1;
                let res_1=self.get_mut_ptr(nindex_1).map(|ptr|{
                    unsafe {
                        &mut *ptr
                    }
                });
                return ((res_0,nindex_0),(res_1,nindex_1));

            });
            return Some(NDimArrayGetMutWithNeiborhoodsMutResult { cur: res_cur, neiborhoods: res_neiborhoods });
        }else {
            return None;
        }
        

    }

    pub fn get_mut_with_neiborhoods_mut_loop<'a>(&'a mut self,index:NDimIndex<DIM>)->Option<NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,Option<&'a mut T>>>{
        let res_cur_ptr_may=self.get_mut_ptr(index);

        if let Some(res_cur_ptr)=res_cur_ptr_may{

            let res_cur=unsafe {
                &mut *res_cur_ptr
            };

            let res_neiborhoods = array::from_fn(|dim|{
                let dim_range=self.n_dim_index.lens()[dim].clone();
                let mut nindex_0=index.clone();
                nindex_0[dim]+=1;
                if dim_range.end<=nindex_0[dim]{
                    nindex_0[dim]=dim_range.start
                }
                let res_0=self.get_mut_ptr(nindex_0).map(|ptr|{
                    unsafe {
                        &mut *ptr
                    }
                });
                let mut nindex_1=index.clone();
                nindex_1[dim]-=1;
                if nindex_0[dim]<dim_range.start{
                    nindex_0[dim]=dim_range.end-1;
                }
                let res_1=self.get_mut_ptr(nindex_1).map(|ptr|{
                    unsafe {
                        &mut *ptr
                    }
                });
                return ((res_0,nindex_0),(res_1,nindex_1));

            });
            return Some(NDimArrayGetMutWithNeiborhoodsMutResult { cur: res_cur, neiborhoods: res_neiborhoods });
        }else {
            return None;
        }
        

    }


    
    pub fn parallel_iter_pair_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,mut func:Func,mut scope_creator:TScopeCreator)
        where Func: FnMut(&'ext_env mut T,&'ext_env mut T,usize)+'ext_env,
        TScopeCreator: ScopeCreator<(),()>
    {
        for mut_dim in 0..DIM {

            struct AScopeUser<'scope,Func,TIndexerIter,const DIM:usize,T,Storage>
                where TIndexerIter:Deref<Target = NDimIndexer<DIM>>,
                Storage:Index<usize,Output=T>+IndexMut<usize>
            {
                values:&'scope mut NDimArray<TIndexerIter,DIM,T,Storage>,
                mut_dim:usize,
                func:&'scope mut Func,
                plus_1:bool,
            };
            impl<'env,Func,TIndexerIter,const DIM:usize,T,Storage> ScopeUser<'env> for AScopeUser<'env,Func,TIndexerIter,DIM,T,Storage> 
                where TIndexerIter:Deref<Target = NDimIndexer<DIM>>,
                Storage:Index<usize,Output=T>+IndexMut<usize>
            {
                type Output=();
            
                fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->Self::Output
                    where TScope:'scope+scope::Scope<'scope,()>,
                        'env:'scope {
                    todo!()
                }
                
                type ScopeFnOutput=();
            }

            scope_creator.scope(
                AScopeUser{values:self,mut_dim,func:&mut func,plus_1:false}
            );
            scope_creator.scope(
                AScopeUser{values:self,mut_dim,func:&mut func,plus_1:true}
            );
        }
    }
    // pub fn parallel_iter_pair_mut<'a,Func,ScopeCreator,Scope>(&'a mut self,func:Func,mut scope_creator:ScopeCreator)
    //     where Func:Fn(&'a mut T,&'a mut T,usize,bool),
    //     ScopeCreator:FnMut( Fn(Scope)  ),
    //     Scope:scope::Scope<'a>
    // {
    //     std::thread::spawn(f)
    // }

    
}

#[derive(Debug)]
pub struct NDimArrayGetWithNeiborhoodsResult<const DIM:usize,T,TNeiborhood>{
    pub cur:T,
    pub neiborhoods:[ ((TNeiborhood,NDimIndex<DIM>),(TNeiborhood,NDimIndex<DIM>)) ;DIM]
}
#[derive(Debug)]
pub struct NDimArrayGetMutWithNeiborhoodsResult<'a,const DIM:usize,T>{
    pub cur:&'a mut T,
    pub neiborhoods:[ ((Option<&'a T>,NDimIndex<DIM>),(Option<&'a T>,NDimIndex<DIM>)) ;DIM]
}
#[derive(Debug)]
pub struct NDimArrayGetMutWithNeiborhoodsMutResult<'a,const DIM:usize,T>{
    pub cur:&'a mut T,
    pub neiborhoods:[ ((Option<&'a mut T>,NDimIndex<DIM>),(Option<&'a mut T>,NDimIndex<DIM>)) ;DIM]
}



#[cfg(test)]
mod test{

    use crate::structures::{n_dim_array::{NDimArray}, n_dim_index::NDimIndexer};

    #[test]
    fn test_ndim_array_bi_iter_mut(){
        const ADIM:usize=3;
        let andidx=NDimIndexer::new_len([0..3,0..3,0..3]);
        let mut andarr=NDimArray::from_fn(&andidx, |idx|idx);
        for awdawd in andidx.iter() {
            let got=andarr.get_mut_with_neiborhoods(awdawd);
            println!("{:#?}",got);
        }
    }
}