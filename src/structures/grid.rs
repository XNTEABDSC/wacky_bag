use super::counted_iter::CountedIterator;

pub struct Grid2D<T>{
    values:Vec<Vec<T>>
}

impl<T> Grid2D<T> {
    
    pub fn new(len:(usize,usize),newfn:impl Fn((usize,usize))->T)->Self {
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
impl<'a,T> IntoIterator for &'a Grid2D<T> {
    type Item=<Self::IntoIter as Iterator>::Item;

    type IntoIter=CountedIterator<std::iter::Map<std::slice::Iter   <'a, Vec<T>>, fn(&    Vec<T>) -> CountedIterator<std::slice::Iter   <'_, T>>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a,T> IntoIterator for &'a mut Grid2D<T> {
    type Item=<Self::IntoIter as Iterator>::Item;

    type IntoIter=CountedIterator<std::iter::Map<std::slice::IterMut<'a, Vec<T>>, fn(&mut Vec<T>) -> CountedIterator<std::slice::IterMut<'_, T>>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}