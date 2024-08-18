use std::marker::PhantomData;

pub trait IdOf<T> {
    fn id(&self)->usize;
}

pub struct IdOf1<T>{
    id:usize,
    p:PhantomData<T>
}

impl<T> IdOf1<T> {
    pub fn new(id:usize)->Self{
        IdOf1{
            id,
            p:PhantomData
        }
    }
    pub fn id(&self)->usize{
        self.id
    }
}

impl<T> IdOf<T> for IdOf1<T> {
    fn id(&self)->usize {
        self.id
    }
}
