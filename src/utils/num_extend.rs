use num_traits::Num;

pub trait NumExtend:Num{
	fn p2()->Self{
		Self::one()+Self::one()
	}
	fn p3()->Self{
		Self::one()+Self::one()+Self::one()
	}
	
}

impl<T:Num> NumExtend for T{

}