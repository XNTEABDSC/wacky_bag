pub struct CountedIterator<TIter>{
    index:usize,
    iter:TIter
}
impl<TIter> CountedIterator<TIter>
where TIter:Iterator{
    pub fn new(iter:TIter)->Self {
        Self{index:0,iter}
    }
}
impl<TIter> Iterator for CountedIterator<TIter>
    where TIter:Iterator
{
    type Item=(usize,<TIter as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let index=self.index;
        self.index+=1;
        self.iter.next().map(|v|{(index,v)})
    }
}