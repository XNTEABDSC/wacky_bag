use num_traits::Num;

pub trait NumExtends:Num{
	fn p2()->Self{
		Self::one()+Self::one()
	}
	fn p3()->Self{
		Self::one()+Self::one()+Self::one()
	}
	fn frac_1_2()->Self{
		Self::one()/(Self::one()+Self::one())
	}
	
}

impl<T:Num> NumExtends for T{

}