use std::marker::PhantomData;

use frunk::Func;

/// You can consider that this is a function about type
/// 
/// auto impl TypeFunc for F:[Func]
pub trait TypeFunc<Input>{
	type Output;
}

// causes conflicting implementations
// impl<F,I,O> TypeFunc<I> for F
// 	where F:Func<I,Output = O>
// {
// 	type Output=O;
// }


// pub trait TypeFuncRev<Output> {
// 	type Input;
// }

// you will find BijectiveTypeFunc<Input,Output> useless as a trait bound that requires both Input and Output

// pub trait BijectiveTypeFunc<Input,Output> : TypeFunc<Input,Output = Output>+TypeFuncRev<Output,Input = Input> {
// }

/// Shows how to find Input via Output
/// 
/// auto impl TypeFunc for F:[BijectiveFunc]
pub trait BijectiveTypeFunc<Output> : TypeFunc<Self::Input,Output = Output> {
	type Input;
}

// causes conflicting implementations
// impl<F,I,O> BijectiveTypeFunc<O> for F
// 	where F: Func<I,Output = O> + BijectiveFunc<O,Input = I> + TypeFunc<I,Output = O>
// {
// 	type Input=I;
// }

/// Shows how to find Input via Output
pub trait BijectiveFunc<Output> : Func<Self::Input,Output = Output> {
	type Input;
	fn inv_call(output:Output)->Self::Input;
}
/// reverse the function if it is bijective
#[derive(Debug,Default,Clone, Copy)]
pub struct ReverseFunc<T>(pub T);
impl<T,I,O> TypeFunc<O> for ReverseFunc<T>
	where T:BijectiveTypeFunc<O,Input = I>//BijectiveTypeFunc<I,O>
{
	type Output=I;
}

impl<T,I,O> BijectiveTypeFunc<I> for ReverseFunc<T>
	where T:TypeFunc<I,Output = O>+BijectiveTypeFunc<O,Input = I>
{
	type Input=O;
}

impl<T,I,O> Func<O> for ReverseFunc<T> 
	where T:BijectiveFunc<O,Input = I>
{
	type Output=I;

	fn call(i: O) -> Self::Output {
		T::inv_call(i)
	}
}

impl<T,I,O> BijectiveFunc<I> for ReverseFunc<T>
	where T:Func<I,Output = O>+BijectiveFunc<O,Input = I>
{
	type Input=O;

	fn inv_call(output:I)->Self::Input {
		T::call(output)
	}
}

/// chain 2 functions together 
/// 
/// call 0 then 1
#[derive(Default,Debug,Clone, Copy)]
pub struct ChainFunc<F1,F2>(pub F1,pub F2);

impl<F1,F2,V1,V2,V3> TypeFunc<V1> for ChainFunc<F1,F2>
	where F1:TypeFunc<V1,Output = V2>,
	F2:TypeFunc<V2,Output = V3>
{
	type Output=V3;
}

impl<F1,F2,V1,V2,V3> BijectiveTypeFunc<V3> for ChainFunc<F1,F2>
	where F2:BijectiveTypeFunc<V3,Input = V2>,
	F1:BijectiveTypeFunc<V2,Input = V1>
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



impl<F1,F2,V1,V2,V3> BijectiveFunc<V3> for ChainFunc<F1,F2> 
	where F2:BijectiveFunc<V3,Input = V2>,
		F1:BijectiveFunc<V2,Input = V1>
{
	type Input = V1;
	
	fn inv_call(output:V3)->Self::Input {
		F1::inv_call(F2::inv_call(output))
	}
}

pub struct FuncAsTypeFunc<F>(pub F);

impl<T,I,O> TypeFunc<I> for FuncAsTypeFunc<T>
	where T:Func<I,Output = O>
{
	type Output=O;
}

impl<T,I,O> BijectiveTypeFunc<O> for FuncAsTypeFunc<T>
	where T:BijectiveFunc<O,Input = I>
{
	type Input=I;
}

/// converts [TypeFunc] into [Func] that uses [PhantomData] as input and output
/// 
/// useful in type expression that can't provide actual call, while can be used by [super::h_list_helpers::HMapP]
pub struct TypeFnAsPhantomFn<F>(pub F);

impl<T,I,O> Func<PhantomData<I>> for TypeFnAsPhantomFn<T>
	where T:TypeFunc<I,Output = O>
{
	type Output=PhantomData<O>;

	fn call(_: PhantomData<I>) -> Self::Output {
		Default::default() 
	}
}

impl<T,I,O> BijectiveFunc<PhantomData<O>> for TypeFnAsPhantomFn<T>
	where T:BijectiveTypeFunc<O,Input = I>
{
	type Input=PhantomData<I>;

	fn inv_call(_output:PhantomData<O>)->Self::Input {
		Default::default() 
	}
}

/// `PhantomData<T>` to `T` and should only be used as type wrapping.
/// 
/// PANICS WHEN USED IN [frunk::HCons::map] AS `mapper`
/// 
/// impl [Func] is provided only to make use of hlist functions type expression but NOT value calculating.
#[derive(Debug,Default,Clone, Copy)]
pub struct MapFromPhantomPanic;

impl<T> Func<PhantomData<T>> for MapFromPhantomPanic {
	type Output=T;

	fn call(_i: PhantomData<T>) -> Self::Output {
		panic!("{:?} should not be called, but only used in type expression",MapFromPhantomPanic)
	}
}

impl<T> BijectiveFunc<T> for MapFromPhantomPanic {
	type Input=PhantomData<T>;

	fn inv_call(_output:T)->Self::Input {
		Default::default()
	}
}

pub type MapPhantomType=MapFromPhantomPanic;