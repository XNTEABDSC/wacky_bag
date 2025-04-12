use std::{array, ops::{Index, IndexMut}};

use super::counted_iter::CountedIterator;

pub struct Grid2D<Container>
    where Container:
    /*
    Index<usize,
        Output: Index<usize,Output = T>
    >+ */
    /*
    IndexMut<usize,
        Output: IndexMut<usize,Output = T>
    >+ */
    /*IntoIterator<Item :IntoIterator<Item=T>>*/
{
    pub values:Container
}
impl<Container>  Grid2D<Container>
{
    pub fn new(values:Container)->Self{
        Self { values }
    }
}
impl<T,Container,TIndex> Index<(TIndex,TIndex)> for Grid2D<Container>
    where Container:
    Index<usize,
        Output: Index<usize,Output = T>
    >,TIndex:Into<usize>
{
    type Output=T;
 
    fn index(&self, index: (TIndex,TIndex)) -> &Self::Output {
        
        &self.values[index.1.into()][index.0.into()]
    }
}
impl<T,Container,TIndex> IndexMut<(TIndex,TIndex)> for Grid2D<Container>
    where Container:
    IndexMut<usize,
        Output: IndexMut<usize,Output = T>
    >,TIndex:Into<usize>
{
 
    fn index_mut(&mut self, index: (TIndex,TIndex)) -> &mut Self::Output {
        &mut self.values[index.1.into()][index.0.into()]
    }
}
impl<Container> IntoIterator for Grid2D<Container> 
    where Container:IntoIterator
{
    type Item=<Container as IntoIterator>::Item;

    type IntoIter=<Container as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
impl<'a,Container> IntoIterator for &'a mut Grid2D<Container> 
    where &'a mut Container:IntoIterator
{
    type Item=<&'a mut Container as IntoIterator>::Item;

    type IntoIter=<&'a mut Container as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
impl<'a ,Container> IntoIterator for &'a Grid2D<Container> 
    where &'a Container:IntoIterator
{
    type Item=<&'a Container as IntoIterator>::Item;

    type IntoIter=<&'a Container as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
/*
pub struct Grid2DArray<T,const XSIZE:usize,const YSIZE:usize>{
    values:[[T;XSIZE];YSIZE]
}

impl<T,const XSIZE:usize,const YSIZE:usize> Grid2DArray<T,XSIZE,YSIZE> {
    pub fn new(mut newfn:impl FnMut((usize,usize))->T)->Self {
        return Self{
            values:array::from_fn(|y|{
                array::from_fn(|x|{
                    newfn((x,y))
                })
            })
        };
    }
    pub fn len(&self)->(usize,usize) {
        (XSIZE,YSIZE)
    }
} */

pub struct Grid2DVec<T>{
    values:Vec<Vec<T>>
}

impl<T> Grid2DVec<T> {
    
    pub fn new(len:(usize,usize),mut newfn:impl FnMut((usize,usize))->T)->Self {
        let (xedge,yedge)=len;
        let mut values:Vec<Vec<T>>=Vec::with_capacity(xedge);
        for x in 0..xedge {
            let mut valuesy:Vec<T>=Vec::with_capacity(yedge);
            for y in 0..yedge {
                valuesy.push(newfn((x,y)));
            }
            values.push(valuesy);
        }
        return Self{
            values
        }
    }
    pub fn len(&self)->(usize,usize) {
        (self.values.len(),self.values.get(0).map_or(0, |v|{v.len()}))
    }

    pub fn get(&self,pos:(usize,usize))->&T {
        &self.values[pos.0][pos.1]
    }
    pub fn get_mut(&mut self,pos:(usize,usize))->&mut T {
        &mut self.values[pos.0][pos.1]
    }
    pub fn iter<'a>(&'a self)         ->CountedIterator<std::iter::Map<std::slice::Iter   <'a, Vec<T>>, fn(&    Vec<T>) -> CountedIterator<std::slice::Iter   <'_, T>>>>{
        CountedIterator::new(self.values.iter().map(|v|{CountedIterator::new(v.iter())}))
    }
    pub fn iter_mut<'a>(&'a mut self) ->CountedIterator<std::iter::Map<std::slice::IterMut<'a, Vec<T>>, fn(&mut Vec<T>) -> CountedIterator<std::slice::IterMut<'_, T>>>>{
        CountedIterator::new(self.values.iter_mut().map(|v|{CountedIterator::new(v.iter_mut())}))
    }
}
impl<'a,T> IntoIterator for &'a Grid2DVec<T> {
    type Item=<Self::IntoIter as Iterator>::Item;

    type IntoIter=CountedIterator<std::iter::Map<std::slice::Iter   <'a, Vec<T>>, fn(&    Vec<T>) -> CountedIterator<std::slice::Iter   <'_, T>>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a,T> IntoIterator for &'a mut Grid2DVec<T> {
    type Item=<Self::IntoIter as Iterator>::Item;

    type IntoIter=CountedIterator<std::iter::Map<std::slice::IterMut<'a, Vec<T>>, fn(&mut Vec<T>) -> CountedIterator<std::slice::IterMut<'_, T>>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}