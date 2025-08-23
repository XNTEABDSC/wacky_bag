use std::{array, ops::{Index, IndexMut, Range}};


#[derive(Debug,Clone, Copy,PartialEq, Eq,PartialOrd, Ord,Hash)]
pub struct ShiftedArray<T,const COUNT:usize,const SHIFT:isize>{
    values:[T;COUNT]
}

impl<T,const COUNT:usize,const SHIFT:isize> ShiftedArray<T,COUNT,SHIFT>  {
    pub fn from_fn<F>(mut cb:F)->Self 
        where F:FnMut(isize)->T
    {
        Self { values: array::from_fn(|i|cb(i as isize+SHIFT)) }
    }

    pub fn get_raw(&self)->&[T;COUNT]{&self.values}
    pub fn get_mut_raw(&mut self)->&mut [T;COUNT]{&mut self.values}
    pub fn into_raw(self)->[T;COUNT]{self.values}
}

impl<T,const COUNT:usize,const SHIFT:isize> Index<isize> for ShiftedArray<T,COUNT,SHIFT>  {
    type Output=T;

    fn index(&self, index: isize) -> &Self::Output {
        &self.values[(index-SHIFT) as usize]
    }
}

impl<T,const COUNT:usize,const SHIFT:isize> IndexMut<isize> for ShiftedArray<T,COUNT,SHIFT>  {

    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        &mut self.values[(index-SHIFT) as usize]
    }
}


#[derive(Debug,Clone,PartialEq, Eq,PartialOrd, Ord,Hash)]
pub struct ShiftedVec<T>{
    values:Vec<T>,
    shift:isize
}

impl<T> ShiftedVec<T>  {
    pub fn from_fn<F>(size:Range<isize>,mut cb:F)->Self 
        where F:FnMut(isize)->T
    {

        Self {shift:size.start, values: size.map(|i|cb(i)).collect() }
    }

    pub fn get_raw(&self)->&Vec<T>{&self.values}
    pub fn get_mut_raw(&mut self)->&mut Vec<T>{&mut self.values}
    pub fn into_raw(self)->Vec<T>{self.values}
}

impl<T> Index<isize> for ShiftedVec<T>  {
    type Output=T;

    fn index(&self, index: isize) -> &Self::Output {
        &self.values[(index-self.shift) as usize]
    }
}

impl<T> IndexMut<isize> for ShiftedVec<T>  {
    
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        &mut self.values[(index-self.shift) as usize]
    }
}