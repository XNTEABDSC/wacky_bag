use crate::structures::n_dim_index::{NDimIndex, NDimIndexer};


pub struct NDimArray<const DIM:usize,T>{
    values:Vec<T>,
    n_dim_index:NDimIndexer<DIM>
}

impl<const DIM:usize,T> NDimArray<DIM,T>  {
    pub fn from_fn<Fn>(n_dim_index:NDimIndexer<DIM>,mut func:Fn)->Self
        where Fn:FnMut(NDimIndex<DIM>)->T
    {
        let index_iter=n_dim_index.iter();
        let values=index_iter.map(|index|func(index)).collect();
        Self { values, n_dim_index }
    }

    pub fn values(&self)->&Vec<T> {
        &self.values
    }

    pub fn n_dim_index(&self)->&NDimIndexer<DIM>{&self.n_dim_index}

    pub fn get(&self,index:NDimIndex<DIM>)->Option<&T>{
        self.values.get(self.n_dim_index.compress_index_u(index))
    }
    pub fn get_mut(&mut self,index:NDimIndex<DIM>)->Option<&mut T>{
        self.values.get_mut(self.n_dim_index.compress_index_u(index))
    }

    pub fn get_with_compressed(&self,index:usize)->Option<&T> {
        self.values.get(index)
    }
    pub fn get_mut_with_compressed(&mut self,index:usize)->Option<&mut T> {
        self.values.get_mut(index)
    }
}