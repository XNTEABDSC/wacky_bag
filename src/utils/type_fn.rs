/// You can consider that this is a function about type
pub trait TypeFunc<Input>{
	type Output;
}
// pub trait TypeFuncRev<Output> {
// 	type Input;
// }

// you will find OneOneMappingTypeFunc<Input,Output> useless as a trait bound that requires both Input and Output

// pub trait OneOneMappingTypeFunc<Input,Output> : TypeFunc<Input,Output = Output>+TypeFuncRev<Output,Input = Input> {
// }
// /// Shows how to find Input via Output
pub trait OneOneMappingTypeFunc<Output> : TypeFunc<Self::Input,Output = Output> {
	type Input;
}

pub struct ReverseTypeFunc<T>(pub T);
impl<T,I,O> TypeFunc<O> for ReverseTypeFunc<T>
	where T:OneOneMappingTypeFunc<O,Input = I>//OneOneMappingTypeFunc<I,O>
{
	type Output=I;
}

impl<T,I,O> OneOneMappingTypeFunc<I> for ReverseTypeFunc<T>
	where T:TypeFunc<I,Output = O>+OneOneMappingTypeFunc<O,Input = I>
{
	type Input=O;
}