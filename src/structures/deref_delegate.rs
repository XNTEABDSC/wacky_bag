//! [`DerefDelegate`]
//! [`DerefMutDelegate`]

use std::ops::{Deref, DerefMut};

/// Behaves like a [`Deref`] by f(guard)
pub struct DerefDelegate<G,F,T>
where F:Fn(&G)->&T
{
    /// guard that somehow can provide `&T`
    pub guard:G,
    /// get `&T` from [`Self::guard`]
    pub f:F
}

impl<G,F,T> Deref for DerefDelegate<G,F,T>
where F:Fn(&G)->&T
{
    type Target=T;

    fn deref(&self) -> &Self::Target {
        (self.f)(&self.guard)
    }
}

/// Behaves like a [`Deref`] and [`DerefMut`] by [`Self::fr`] and [`Self::fm`]
pub struct DerefMutDelegate<G,FR,FM,T>
where FR:Fn(&G)->&T,
    FM:FnMut(&mut G)->&mut T
{
    /// guard that somehow can provide `&mut T`
    pub guard:G,
    /// get `&T` from [`Self::guard`]
    pub fr:FR,
    /// get `&mut T` from [`Self::guard`]
    pub fm:FM
}

impl<G,FR,FM,T> Deref for DerefMutDelegate<G,FR,FM,T>
where FR:Fn(&G)->&T,
    FM:FnMut(&mut G)->&mut T
{
    type Target=T;

    fn deref(&self) -> &Self::Target {
        (self.fr)(&self.guard)
    }
}

impl<G,FR,FM,T> DerefMut for DerefMutDelegate<G,FR,FM,T>
where FR:Fn(&G)->&T,
    FM:FnMut(&mut G)->&mut T
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        
        (self.fm)(&mut self.guard)
    }
}