pub trait PartialMapper<Input,Marker>{
	type Output;
	fn try_map(&self,input:Input)->Option<Self::Output>;
}

pub trait PartialMapperBi<A,B,Marker>
	where Self:PartialMapper<A,Self::MA,Output = B>+PartialMapper<B,Self::MB,Output = A>,
{
	type MA;
	type MB;
	fn try_a_2_b(&self,a:A)->Option<B>{
		PartialMapper::<A,Self::MA>::try_map(self, a)
	}

	fn try_b_2_a(&self,b:B)->Option<A>{
		PartialMapper::<B,Self::MB>::try_map(self, b)
	}
}

pub struct NumMarker<const N:isize>;