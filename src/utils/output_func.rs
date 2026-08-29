//! [`HMappableFrom`]
use frunk::{Func, HCons, HNil, Poly};

use crate::utils::type_fn::BijectiveTypeFunc;

// pub trait BijectiveFunc<Output> : Func<Self::Input,Output = Output> {
// 	type Input;
// }

/// Similar to [frunk::hlist::HMappable], but infer input type via output type.
/// 
/// This is useful when you need to do someting after [frunk::hlist::HCons::sculpt]
/// 
/// requires `OutputMapper` to implement [BijectiveTypeFunc]
pub trait HMappableFrom<OutputMapper> {
	/// type of input which will be mapped to output (self)
	type Input;
	/// map input to output
	fn output_map(input:Self::Input,mapper:OutputMapper)->Self;
}

impl<Mapper> HMappableFrom<Poly<Mapper>> for HNil {
	type Input=HNil;

	fn output_map(_:HNil,_mapper:Poly<Mapper>)->HNil {
		HNil
	}
}

impl<Mapper,InputH,InputT,OutputH,OutputT> HMappableFrom<Poly<Mapper>> for HCons<OutputH,OutputT>
	where Mapper:BijectiveTypeFunc<OutputH,Input = InputH> + Func<InputH,Output=OutputH>,
	OutputT:HMappableFrom<Poly<Mapper>,Input = InputT>
{
	type Input = HCons<InputH,InputT>;
	fn output_map(input:Self::Input,mapper:Poly<Mapper>)->HCons<OutputH,OutputT> {
		HCons { head: Mapper::call(input.head), tail: HMappableFrom::output_map(input.tail, mapper) }
	}
}

// pub type HMapFP<Output,Input,Mapper>=<Output as HMappableFrom<Mapper,Input = Input>>::Output;
#[cfg(test)]
mod test{
    use std::ops::{Add, AddAssign};

use frunk::{HList, Poly, hlist, hlist_pat};

use crate::{impl_func_closure, new_new_type_func, new_struct_func, utils::{output_func::HMappableFrom, type_fn::ReverseFunc}};

	struct NTS<T>(pub T);
	new_new_type_func!(NTS MapS);
	struct NTC<T>(pub T);
	new_new_type_func!(NTC MapC);
	struct A(pub i32);
	struct B(pub i32);
	struct C(pub i32);
	fn sum_a_c(hlist_pat![a,c]:HList!(A,C))->i32{
		a.0+c.0
	}
	#[test]
	fn test(){
		let l=hlist!(NTS(A(1)),NTC(A(2)),NTS(B(3)),NTC(B(4)),NTS(C(5)),NTC(C(6)));
		assert_eq!(
			sum_a_c(
				HMappableFrom::output_map(
					l.sculpt().0, 
					Poly(ReverseFunc(MapC)))),
			2+6);
	}
}