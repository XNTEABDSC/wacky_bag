use std::{any::Any, array, collections::HashMap, sync::{LazyLock, Mutex}};

use crate::{structures::n_dim_array::{n_dim_array::NDimArray, n_dim_index::NDimIndex, n_dim_indexer_u::NDimIndexerU, t_n_dim_indexer::TNDimIndexer}, utils::dim_root_of_x_usize::get_dim_root_of_x_usize};

pub type NDimChunk<T,const  DIM:usize>=NDimArray<&'static NDimIndexerU<DIM>,DIM,T,Vec<T>>;

// pub struct NDimChunk_<T,const DIM:usize>{
//     values:Vec<T>,
//     indexer:&'static NDimIndexerU<DIM>
// }

static CACHED_N_DIM_INDEXER_U:LazyLock<Mutex<HashMap<(usize,usize),Box<dyn Any+Send>>>>=LazyLock::new(||Default::default());

pub fn get_cached_n_dim_indexer_u<const DIM:usize>(dim_elem_count:usize)->&'static NDimIndexerU<DIM>{
    let key=(DIM,dim_elem_count);
    let mut cached=CACHED_N_DIM_INDEXER_U.lock().unwrap();
    

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


// static CACHED_N_DIM_INDEXER:LazyLock<Mutex<HashMap<(usize,usize),Box<dyn Any+Send>>>>=LazyLock::new(||Default::default());

// pub fn get_cached_n_dim_indexer<const DIM:usize>(dim_elem_count:usize)->&'static NDimIndexer<DIM>{
//     let key=(DIM,dim_elem_count);
//     let mut cached=CACHED_N_DIM_INDEXER.lock().unwrap();
//     if let Some(res)=cached.get(&key){
//         return unsafe {
//             &*(res.downcast_ref::<NDimIndexer<DIM>>().unwrap() as *const NDimIndexer<DIM>)
//         };
//     }
//     else
//     {
//         let res=Box::new(NDimIndexer::<DIM>::new_len(array::from_fn(|_|0..(dim_elem_count as isize))));
//         let res_in=cached.entry(key).or_insert(res);
//         return unsafe {
//             &*(res_in.downcast_ref::<NDimIndexer<DIM>>().unwrap() as *const NDimIndexer<DIM>)
//         };
//     }
// }

pub fn get_chunk_dim_elem_count<T,const DIM:usize>(chunk_size:usize)->(usize,usize){
    let size_of_t=std::mem::size_of::<T>();
    let chunk_count=chunk_size/size_of_t;
    let (dim_elem_count,chunk_elem_count)=get_dim_root_of_x_usize(chunk_count,DIM);
    return (dim_elem_count,chunk_elem_count);
}

pub fn get_chunk_n_dim_indexer_u<T,const DIM:usize>(chunk_size:usize)->&'static NDimIndexerU<DIM>{
    let dim_elem_count=get_chunk_dim_elem_count::<T,DIM>(chunk_size).0;
    get_cached_n_dim_indexer_u::<DIM>(dim_elem_count)
}

/// 32KB
pub const COMMON_CHUNK_SIZE:usize=32*1024;

pub fn from_fn<T,const DIM:usize,Func>(mut f:Func,chunk_size:usize)->NDimChunk<T,DIM>
	where Func:FnMut(NDimIndex<DIM>)->T
{
	let indexer=get_chunk_n_dim_indexer_u::<T,DIM>(chunk_size);

	let values=Vec::from_iter(indexer.iter().map(|idx|f(idx)));//Vec::from_iter();
	
	NDimArray::new(indexer, values)
}

// impl<T,const DIM:usize> NDimChunk_<T,DIM> {
//     pub fn from_fn<Func>(chunk_size:usize,mut f:Func)->Self
//         where Func:FnMut(NDimIndexU<DIM>)->T
//     {
//         let size_of_t=size_of::<T>();
//         let chunk_count=chunk_size/size_of_t;
//         let (dim_elem_count,chunk_elem_count)=get_dim_root_of_x_usize(chunk_count,DIM);
//         let indexer=get_cached_n_dim_indexer_u::<DIM>(dim_elem_count);

//         let values=Vec::from_iter(indexer.iter().map(|idx|f(idx)));//Vec::from_iter();
        
//         Self { values ,indexer}
//     }
// }

// impl<T,const DIM:usize> Index<NDimIndexU<DIM>> for NDimChunk_<T,DIM> {
//     type Output = T;

//     fn index(&self, index: NDimIndexU<DIM>) -> &Self::Output {
//         &self.values[self.indexer.compress_index(index)]
//     }
// }

// impl<T,const DIM:usize> IndexMut<NDimIndexU<DIM>> for NDimChunk_<T,DIM> {
// 	fn index_mut(&mut self, index: NDimIndexU<DIM>) -> &mut Self::Output {
// 		&mut self.values[self.indexer.compress_index(index)]
// 	}
	
// }