use std::{marker::PhantomData, ops::Index};

/// impl Index for F
pub struct FnAsIndex<'a,F>(pub F,pub PhantomData<&'a F>);

impl<'a, F> FnAsIndex<'a, F> {
    /// impl Index for F
    pub fn new(_0: F) -> Self {
        Self(_0, PhantomData)
    }
}

impl<'a,F,I,O:'a> Index<I> for FnAsIndex<'a,F>
where F:Fn(I)->&'a O
{
    type Output=O;

    fn index(&self, index: I) -> &Self::Output {
        (self.0)(index)
    }
}