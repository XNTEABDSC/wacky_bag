use std::ops::Add;

use frunk::{ToRef, hlist::Sculptor};



pub fn h_extend_by_fn<V,Idx,F,FI,FO,O>(v:V,f:F)->O
where F:FnOnce(FI)->FO,
	V:Sculptor<FI,Idx,Remainder : Add<FO,Output = O>>,
{
	let (fi,r)=v.sculpt();
	let fo=f(fi);
	r+fo
}

// pub fn h_extend_by_fn_ref<V,Idx,S,F,FI,FO,O>(v:V,f:F)->O
// where F:FnOnce(FI)->FO,
// 	V:Sculptor<S,Idx,Remainder : Add<FO,Output = O>>,
// 	S:ToRef<Output = FI>
// {
// 	let (fi,r)=v.sculpt();
// 	let fo=f(fi.to_ref());
// 	r+fo
// }

// pub fn h_apply_fn_inplace<>