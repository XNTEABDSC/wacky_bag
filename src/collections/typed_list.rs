

//$crate::collections::typed_list::
#[macro_export]
macro_rules! make_typed_list {
    (  $type_name:ident,$trait_name:ident )=>{
        //use $type_name;
        /*
        mod $type_name{
            use super::*;
            
        }
        */
            pub struct $type_name{
                pub values:Vec<*mut dyn $trait_name>
            }
            impl $type_name{
                pub fn new()->Self{
                    $type_name{
                        values:Vec::new()
                    }
                }
                pub fn add<'a,T:'a>(&mut self, elem:T)->$crate::collections::typed_list::IdOf<T>
                    where T:$trait_name
                {
                    return self.add_boxed(Box::new(elem));
                }
                pub fn add_boxed<'a,T:'a>(&mut self, elem:Box<T>)->$crate::collections::typed_list::IdOf<T>
                    where T:$trait_name
                {
                    let id=self.values.len();
                    let cast_elem:*mut dyn $trait_name=(Box::into_raw(elem)) as *mut dyn $trait_name;
                    //Box::<T>::into_raw(elem) as *mut TBase;
                    self.values.push(cast_elem);
                    return $crate::collections::typed_list::IdOf::<T>::new(id);
                }
                pub fn get<'a,T>(&'a self,id:$crate::collections::typed_list::IdOf<T>)->&'a mut T
                    where T:$trait_name
                {
                    return unsafe{ &mut * ( 
                        self.values[id.id()] as *mut T
                    )};
                }
            }
            impl Drop for $type_name {
                fn drop(&mut self) {
                    for i in &( self.values ){
                        unsafe{ drop(Box::from_raw(*i))}
                    }
                }
            }

    }
}

pub struct IdOf<T:?Sized>{
    id:usize,
    p: std::marker::PhantomData<T>,
}

impl<T:?Sized> IdOf<T> {

    pub fn new(id:usize)->Self{
        IdOf{
            id,
            p:std::marker::PhantomData
        }
    }

    pub fn id(&self)->usize{
        self.id
    }
}

/*
pub struct TypedList<TBase:?Sized>{
    values:Vec<*mut TBase>
}

pub struct IdOf<T:?Sized>{
    id:usize,
    p:PhantomData<T>,
}

impl<T:?Sized> IdOf<T> {

    pub fn new(id:usize)->Self{
        IdOf{
            id,
            p:PhantomData
        }
    }

    pub fn id(&self)->usize{
        self.id
    }
}

impl<TBase:?Sized> TypedList<TBase>{
    pub fn new()->Self{
        TypedList{
            values:Vec::new()
        }
    }


    pub fn add<'a,T:'a>(&mut self, elem:T)->IdOf<T>
        where *mut TBase : From <* mut T>,
            T:Into<TBase>
        //where * mut T:*mut TBase
    {
        /*
        let id=self.values.len();
        let cast_elem:*mut TBase=(Box::into_raw(Box::new(elem)) ).into();
        //Box::<T>::into_raw(elem) as *mut TBase;
        self.values.push(cast_elem);
        return IdOf::<T>::new(id);
         */
        return self.add_boxed(Box::new(elem));
    }
    
    pub fn add_boxed<'a,T:'a+?Sized>(&mut self, elem:Box<T>)->IdOf<T>
        where *mut TBase : From <* mut T>
    {
        let id=self.values.len();
        let cast_elem:*mut TBase=(Box::into_raw(elem)).into();
        //Box::<T>::into_raw(elem) as *mut TBase;
        self.values.push(cast_elem);
        return IdOf::<T>::new(id);
    }
    
    pub fn get<'a,T:?Sized>(&'a self,id:IdOf<T>)->&'a mut T
        where *mut TBase : TryInto <* mut T>
    {
        return unsafe{ &mut * ( 
            match self.values[id.id()].try_into(){
                Ok(v)=>v,
                Err(_)=>panic!("cast error")
            } 
        )};//unsafe {self.values[id.id()].as_mut()}.unwrap().into();
    }
}

impl<T:?Sized> Drop for TypedList<T> {
    fn drop(&mut self) {
        for i in &self.values{
            unsafe{ drop(Box::from_raw(*i))}
        }
    }
}


 */