use std::ops::{Deref, Range};

use crate::structures::{n_dim_array::TNDimArray, n_dim_chunk::NDimChunk, n_dim_index::{NDimIndex, NDimIndexer, TNDimIndex}};

use crate::structures::n_dim_chunk;



pub struct NDimChunkArray<const DIM:usize,T,TIndexer> 
    where TIndexer:Deref<Target = NDimIndexer<DIM>>,
{
    values:Vec<NDimChunk<T,DIM>>,
    n_dim_index:TIndexer,
    dim_elem_count:usize,
    chunk_elem_count:usize,
}

impl<const DIM:usize,T,TIndexer> NDimChunkArray<DIM,T,TIndexer> 
    where TIndexer:Deref<Target = NDimIndexer<DIM>>,
{
    pub fn from_fn(n_dim_index:TIndexer,mut func:impl FnMut(&NDimIndex<DIM>)->T)->Self {
        let (dim_elem_count,chunk_elem_count)=n_dim_chunk::get_chunk_dim_elem_count::<T,DIM>(n_dim_chunk::COMMON_CHUNK_SIZE);
        let length=n_dim_index.length_u();
        // let n_len

        Self {
            values:todo!(),
            n_dim_index,
            dim_elem_count,
            chunk_elem_count
        }

    }
    
}

impl<TIndexer,const DIM:usize,T> TNDimArray<DIM, T> for NDimChunkArray<TIndexer,DIM,T>
where
    TIndexer:Deref<Target = NDimIndexer<DIM>>,
{
    fn lens(&self)->&[std::ops::Range<isize>;DIM] {
        self.n_dim_index.lens()
    }

    fn get(&self,indexes:&super::n_dim_index::NDimIndex<DIM>)->Option<&T> {
        todo!()
    }

    fn get_mut(&mut self,indexes:&super::n_dim_index::NDimIndex<DIM>)->Option<&mut T> {
        todo!()
    }

    fn get_with_neiborhoods_generic<'a,ForNeiborhood,NeiborhoodResult>(&'a self,index:&super::n_dim_index::NDimIndex<DIM>,for_neiborhood:ForNeiborhood)->Option<super::n_dim_array::NDimArrayGetWithNeiborhoodsResult<DIM,&'a T,NeiborhoodResult>>
            where ForNeiborhood:Fn(&'a Self,&mut super::n_dim_index::NDimIndex<DIM>,usize,bool)->NeiborhoodResult {
        todo!()
    }

    fn get_mut_with_neiborhoods_generic<'a,ForNeiborhood,NeiborhoodResult>(&'a mut self,index:&super::n_dim_index::NDimIndex<DIM>,for_neiborhood:ForNeiborhood)->Option<super::n_dim_array::NDimArrayGetWithNeiborhoodsResult<DIM,&'a mut T,NeiborhoodResult>>
            where ForNeiborhood:FnMut(&mut Self,&mut super::n_dim_index::NDimIndex<DIM>,usize,bool)->NeiborhoodResult {
        todo!()
    } 

    fn parallel_iter_pair_mut<'ext_env,Func,TScopeCreator>(&'ext_env mut self,func:Func,scope_creator:TScopeCreator)
        where Func: for<'scope> Fn(&'scope mut T,&'scope mut T,usize)+'ext_env+Sync+Send,
        TScopeCreator: crate::traits::scope::ThreadScopeCreator<(),()>,
            T:Send+Sync {
        todo!()
    }
}