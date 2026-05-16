pub trait SingleFn<M>{
	// type Marker=M;
	type Input;
	type Output;
	fn call(self,input:Self::Input)->Self::Output;
}

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