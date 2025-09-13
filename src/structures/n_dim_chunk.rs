

pub struct NDimChunk<T,const DIM:usize>{
    values:Vec<T>
}


impl<T,const DIM:usize> NDimChunk<T,DIM> {
    pub fn from_fn<Func>(f:Func)
        where Func:FnMut()
    {

    }
}