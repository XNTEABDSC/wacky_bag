use std::{marker::PhantomData, ops::{AddAssign, Mul, MulAssign}};



//mod test_type_list;
mod test_bind_to;
mod test_int_list;
mod test_tuple;
mod test_grid_iter;

pub trait SingleFunc{
	type Input;
	type Output;
	fn call(input:Self::Input)->Self::Output;
}

pub fn sq<T>(v:T)->T
	where T:Mul<Output = T>+Copy
{
	v*v
}

pub struct Sq<T:Mul<Output = T>+Copy>(pub PhantomData<T>);

impl<T:Mul<Output = T>+Copy> SingleFunc for Sq<T> {
	type Input=T;

	type Output=T;

	fn call(input:Self::Input)->Self::Output {
		sq(input)
	}
}