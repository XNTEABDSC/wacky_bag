//! [`impl_phantom`]

/// impl Debug, Hash, Eq, Clone, Default for `struct bla<T>(PhantomType<T>)`
#[macro_export]
macro_rules! impl_phantom {
	($name:ident< $($g:tt),* > $(where $($tt:tt)+)?) => {
impl<$($g),*> Copy for $name<$($g),*> $(where $($tt)+)?{}

impl<$($g),*> std::fmt::Debug for $name<$($g),*> $(where $($tt)+)?{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple( std::any::type_name::<Self>() ).field(&self.0).finish()
    }
}

impl<$($g),*> std::hash::Hash for $name<$($g),*> $(where $($tt)+)?{
    fn hash<__Hasher: std::hash::Hasher>(&self, state: &mut __Hasher) {
        self.0.hash(state);
    }
}

impl<$($g),*> std::cmp::Eq for $name<$($g),*> $(where $($tt)+)?{}

impl<$($g),*> std::cmp::PartialEq for $name<$($g),*> $(where $($tt)+)?{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<$($g),*> std::clone::Clone for $name<$($g),*> $(where $($tt)+)?{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<$($g),*> std::default::Default for $name<$($g),*> $(where $($tt)+)?{
    fn default() -> Self {
        Self(Default::default())
    }
}
	};
}