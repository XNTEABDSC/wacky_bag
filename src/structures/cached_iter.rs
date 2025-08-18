use std::mem;

pub struct CachedIter<T,TIter:Iterator<Item = T>>{
    current:Option<T>,
    iter:TIter
}

impl<T,TIter:Iterator<Item = T>> CachedIter<T,TIter> {
    pub fn new(mut iter:TIter)->Self{
        let current=iter.next();
        Self { current, iter }
    }

    pub fn get(&self)->Option<&T>{
        self.current.as_ref()
    }

    pub fn unwrap(self)->(Option<T>,TIter) {
        (self.current,self.iter)
    }

}

impl<T,TIter:Iterator<Item = T>> Iterator for CachedIter<T,TIter> {
    type Item=T;

    fn next(&mut self) -> Option<Self::Item> {
        mem::replace(&mut self.current, self.iter.next())
    }
}