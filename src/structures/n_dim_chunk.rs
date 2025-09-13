use std::{collections::HashMap, sync::{Arc, LazyLock, Mutex}};

use crate::utils::dim_root_of_x_usize::get_dim_root_of_a_usize;



pub struct NDimChunk<T,const DIM:usize>{
    values:Vec<T>
}


impl<T,const DIM:usize> NDimChunk<T,DIM> {
    pub fn from_fn<Func>(chunk_size:usize,f:Func)->Self
        where Func:FnMut()
    {
        let size_of_t=size_of::<T>();
        let chunk_count=chunk_size/size_of_t;
        let (dim_count,chunk_count)=get_dim_root_of_a_usize(chunk_count,DIM);
        let mut values=Vec::from_iter(iter);
        Self { values }
    }
}