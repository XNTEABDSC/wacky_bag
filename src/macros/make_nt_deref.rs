// not docing things that is not important (not used and may be even I forgot what they does)
#![allow(missing_docs)]

/// used for a pattern but not used
#[macro_export]
macro_rules! make_nt_deref {
    ($name_type:ident,$name_trait:ident,$name_method:ident) => {
//#[derive(Clone,Copy)]
/// you can convert ref into this type, then impl for this type.
pub struct $name_type<TRef,TValue>(pub TRef,std::marker::PhantomData<TValue>);

impl<TRef,TValue> $name_type<TRef,TValue> {
	/// new
    pub fn new(a:TRef)->Self {
        Self(a, Default::default())
    }
}
/// to convert a reference into $name_type
pub trait $name_trait<TValue>
    where Self:std::ops::Deref<Target = TValue>+Sized
{
	/// convert the reference into $name_type
    fn $name_method(self)->$name_type<Self,TValue>;
}

impl<TRef,TValue> $name_trait<TValue> for TRef 
    where TRef:std::ops::Deref<Target = TValue>+Sized
{
    fn $name_method(self)->$name_type<Self,TValue> {
        $name_type::new(self)
    }
}

impl<TRef,TValue> Clone for $name_type<TRef,TValue>
    where TRef:Clone
{
    fn clone(&self)->Self{
        Self(self.0.clone(),self.1.clone())
    }
}
impl<TRef,TValue> Copy for $name_type<TRef,TValue>
    where TRef:Copy
{

}
    };
}