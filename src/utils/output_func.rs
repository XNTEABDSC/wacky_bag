
use frunk::{Func, HCons, HNil, Poly};

use crate::utils::type_fn::OneOneMappingTypeFunc;

// pub trait OneOneMappingFunc<Output> : Func<Self::Input,Output = Output> {
// 	type Input;
// }

pub trait HMappableFrom<OutputMapper> {
	type Input;
	fn output_map(input:Self::Input,mapper:OutputMapper)->Self;
}

impl<Mapper> HMappableFrom<Poly<Mapper>> for HNil {
	type Input=HNil;

	fn output_map(_:HNil,_mapper:Poly<Mapper>)->HNil {
		HNil
	}
}

impl<Mapper,InputH,InputT,OutputH,OutputT> HMappableFrom<Poly<Mapper>> for HCons<OutputH,OutputT>
	where Mapper:OneOneMappingTypeFunc<OutputH,Input = InputH> + Func<InputH,Output=OutputH>,
	OutputT:HMappableFrom<Poly<Mapper>,Input = InputT>
{
	type Input = HCons<InputH,InputT>;
	fn output_map(input:Self::Input,mapper:Poly<Mapper>)->HCons<OutputH,OutputT> {
		HCons { head: Mapper::call(input.head), tail: HMappableFrom::output_map(input.tail, mapper) }
	}
}