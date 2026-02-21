pub struct SPoly<T>(pub T);

pub trait SFunc<Input>{
	type Output;
	fn call(self,input:Input)->Output;
}

