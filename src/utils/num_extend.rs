//! [`NumExtends`]

use num_traits::Num;
/// [`NumExtends::p2`] [`NumExtends::p3`] [`NumExtends::frac_1_2`]
pub trait NumExtends:Num{
	/// `2`
	fn p2()->Self{
		Self::one()+Self::one()
	}
	/// `3`
	fn p3()->Self{
		Self::one()+Self::one()+Self::one()
	}
	/// `1/2`
	fn frac_1_2()->Self{
		Self::one()/(Self::one()+Self::one())
	}
	
}

impl<T:Num> NumExtends for T{

}