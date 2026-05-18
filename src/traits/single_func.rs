//! [SingleFn]


/// A trait that can get Input(single param) and Output of a function by rust infer
pub trait SingleFn<M>{
	/// input (single param) of the function
	type Input;
	/// output of the function
	type Output;
	/// call the function
	fn call(self,input:Self::Input)->Self::Output;
}
/// a static fn 
/// 
/// `(I,O)` `fn(I)->O`
pub struct SingleFnStaticMarker<T>(pub T);

impl<I,O> SingleFn<SingleFnStaticMarker<(I,O)>> for fn(I)->O
	// where F:Fn(I)->O
{
	type Input=I;

	type Output=O;

	fn call(self,input:Self::Input)->Self::Output {
		self(input)
	}
}

/// a [`Fn`]
/// 
/// `(I,O)` `Fn(I)->O`
pub struct SingleFnMarker<T>(pub T);

impl<'a,F,I,O> SingleFn<SingleFnMarker<(I,O)>> for &'a F
	where F:Fn(I)->O
{
	type Input=I;

	type Output=O;

	fn call(self,input:Self::Input)->Self::Output {
		self(input)
	}
}

/// a [FnMut]
/// 
/// `(I,O)` `FnMut(I)->O`
pub struct SingleFnMutMarker<T>(pub T);

impl<'a,F,I,O> SingleFn<SingleFnMarker<(I,O)>> for &'a mut F
	where F:FnMut(I)->O
{
	type Input=I;

	type Output=O;

	fn call(self,input:Self::Input)->Self::Output {
		self(input)
	}
}

/// a [`FnOnce`]
/// 
/// `(I,O)` `FnOnce(I)->O`
pub struct SingleFnOnceMarker<T>(pub T);

impl<F,I,O> SingleFn<SingleFnOnceMarker<(I,O)>> for F
	where F:FnOnce(I)->O
{
	type Input=I;

	type Output=O;

	fn call(self,input:Self::Input)->Self::Output {
		self(input)
	}
}