use frunk::Func;

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

pub trait OneOneMappingFunc<Output> : Func<Self::Input,Output = Output> {
	type Input;
	fn inv_call(output:Output)->Self::Input;
}

pub struct ReverseFunc<T>(pub T);
impl<T,I,O> TypeFunc<O> for ReverseFunc<T>
	where T:OneOneMappingTypeFunc<O,Input = I>//OneOneMappingTypeFunc<I,O>
{
	type Output=I;
}

impl<T,I,O> OneOneMappingTypeFunc<I> for ReverseFunc<T>
	where T:TypeFunc<I,Output = O>+OneOneMappingTypeFunc<O,Input = I>
{
	type Input=O;
}

impl<T,I,O> Func<O> for ReverseFunc<T> 
	where T:OneOneMappingFunc<O,Input = I>
{
	type Output=I;

	fn call(i: O) -> Self::Output {
		T::inv_call(i)
	}
}

impl<T,I,O> OneOneMappingFunc<I> for ReverseFunc<T>
	where T:Func<I,Output = O>+OneOneMappingFunc<O,Input = I>
{
	type Input=O;

	fn inv_call(output:I)->Self::Input {
		T::call(output)
	}
}
#[derive(Default,Debug,Clone, Copy)]
pub struct ChainFunc<F1,F2>(pub F1,pub F2);

impl<F1,F2,V1,V2,V3> TypeFunc<V1> for ChainFunc<F1,F2>
	where F1:TypeFunc<V1,Output = V2>,
	F2:TypeFunc<V2,Output = V3>
{
	type Output=V3;
}

impl<F1,F2,V1,V2,V3> OneOneMappingTypeFunc<V3> for ChainFunc<F1,F2>
	where F2:OneOneMappingTypeFunc<V3,Input = V2>,
	F1:OneOneMappingTypeFunc<V2,Input = V1>
{
	type Input = V1;
}

impl<F1,F2,V1,V2,V3> Func<V1> for ChainFunc<F1,F2> 
	where F1:Func<V1,Output = V2>,
	F2:Func<V2,Output = V3>
{
	type Output=V3;

	fn call(i: V1) -> Self::Output {
		F2::call(F1::call(i))
	}
}



impl<F1,F2,V1,V2,V3> OneOneMappingFunc<V3> for ChainFunc<F1,F2> 
	where F2:OneOneMappingFunc<V3,Input = V2>,
		F1:OneOneMappingFunc<V2,Input = V1>
{
	type Input = V1;
	
	fn inv_call(output:V3)->Self::Input {
		F1::inv_call(F2::inv_call(output))
	}
}