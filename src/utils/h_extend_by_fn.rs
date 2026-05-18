//! [`h_extend_by_fn`]
//! [`h_extend_by_fn_ref`]

use std::ops::Add;

use frunk::{ToRef, hlist::Sculptor};


/// `f` sculpt params `FI` from `v` and generates `FO`
pub fn h_extend_by_fn<V,Idx,F,FI,FO,O>(v:V,f:F)->O
where F:FnOnce(FI)->FO,
	V:Sculptor<FI,Idx,Remainder : Add<FO,Output = O>>,
{
	let (fi,r)=v.sculpt();
	let fo=f(fi);
	r+fo
}

/// `f` sculpt params `FI` from `v.to_ref()` and generates `FO`
pub fn h_extend_by_fn_ref<V,Idx,F,FI,FO,O>(v:V,f:F)->O
where 
	F:FnOnce(FI)->FO,
	V:Add<FO,Output = O>,
	V:for<'a> ToRef<'a,Output : Sculptor<FI,Idx>>,
{
	let fi=v.to_ref().sculpt().0;
	let fo=f(fi);
	v+fo
}

// pub fn h_apply_fn_inplace<>