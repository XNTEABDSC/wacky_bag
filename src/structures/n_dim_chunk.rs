use std::{any::Any, collections::HashMap, sync::{Arc, LazyLock, Mutex}};

use crate::{structures::n_dim_index_u::{NDimIndexU, NDimIndexerU}, utils::dim_root_of_x_usize::get_dim_root_of_a_usize};



pub struct NDimChunk<T,const DIM:usize>{
    values:Vec<T>,
    indexer:&'static NDimIndexerU<DIM>
}

static CACHED_N_DIM_INDEXER:LazyLock<Mutex<HashMap<(usize,usize),Box<dyn Any+Send>>>>=LazyLock::new(||Default::default());

pub fn get_cached_n_dim_indexer_u<const DIM:usize>(dim_elem_count:usize)->&'static NDimIndexerU<DIM>{
    let key=(DIM,dim_elem_count);
    let mut cached=CACHED_N_DIM_INDEXER.lock().unwrap();
    

    if let Some(res)=cached.get(&key){
        return unsafe {
            &*(res.downcast_ref::<NDimIndexerU<DIM>>().unwrap() as *const NDimIndexerU<DIM>)
        };
    }
    else
    {
        let res=Box::new(NDimIndexerU::new_len([dim_elem_count;DIM]));
        
        let res_in=cached.entry(key).or_insert(res);
        return unsafe {
            &*(res_in.downcast_ref::<NDimIndexerU<DIM>>().unwrap() as *const NDimIndexerU<DIM>)
        };
    }
}


impl<T,const DIM:usize> NDimChunk<T,DIM> {
    pub fn from_fn<Func>(chunk_size:usize,mut f:Func)->Self
        where Func:FnMut(NDimIndexU<DIM>)->T
    {
        let size_of_t=size_of::<T>();
        let chunk_count=chunk_size/size_of_t;
        let (dim_elem_count,chunk_elem_count)=get_dim_root_of_a_usize(chunk_count,DIM);
        let indexer=get_cached_n_dim_indexer_u::<DIM>(dim_elem_count);

        let values=Vec::from_iter(indexer.iter().map(|idx|f(idx)));//Vec::from_iter();
        
        Self { values ,indexer}
    }
}