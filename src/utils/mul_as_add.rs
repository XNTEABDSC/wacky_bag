// use std::ops::{Add, AddAssign, Mul, MulAssign};


// #[derive(Debug,Clone, Copy)]
// pub struct MulAsAdd<T>(pub T);

// impl<T,T2> Add<MulAsAdd<T2>> for MulAsAdd<T>
// 	where T:Mul<T2>
// {
// 	type Output=MulAsAdd<T::Output>;

// 	fn add(self, rhs: MulAsAdd<T2>) -> Self::Output {
// 		MulAsAdd(self.0*rhs.0)
// 	}
// }


// impl<T,T2> AddAssign<MulAsAdd<T2>> for MulAsAdd<T>
// 	where T:MulAssign<T2>
// {	
// 	fn add_assign(&mut self, rhs: MulAsAdd<T2>) {
// 		self.0*=rhs.0
// 	}
// }

