

//pub trait GetHasMult<HasMarkers> {
//	type Output<'a>;
//	fn get_mult<'a>(&'a self)->
//}


//pub struct GetMult<HasMarkers>(pub HasMarkers);

use frunk::{HCons, HNil};

use crate::traits::has::{Has, HasMarker};
/// Self that matchs multiple [`Has`] . 
pub trait HasMult<HasMarkers>
{
	type Output;
	fn get_has_mult(self,markers:HasMarkers)->Self::Output;
}

impl<This> HasMult<HNil> for This 
{
	type Output=HNil;

	fn get_has_mult(self,_markers:HNil)->Self::Output {
		HNil
	}
}

impl<MarkerH,MarkerT,This,ThisGetMHOutput,ThisGetMSOutput> HasMult<HCons<MarkerH,MarkerT>> for This
	where 
		This:Clone,
		MarkerH:HasMarker<Item = ThisGetMHOutput>,
		This:Has<MarkerH>,
		This:HasMult<MarkerT,Output = ThisGetMSOutput>
{
	type Output=HCons<ThisGetMHOutput,ThisGetMSOutput>;

	fn get_has_mult(self,markers:HCons<MarkerH,MarkerT>)->Self::Output {
		HCons{
			head:Has::<MarkerH>::get_has(self.clone(), markers.head),
			tail:HasMult::<MarkerT>::get_has_mult(self,markers.tail)
		}
	}
}


#[cfg(test)]
mod test{
	use std::{marker::PhantomData, ops::Deref};

	use frunk::{HList, hlist};
	

	use crate::structures::just::Just;

	use super::*;
	
	
	struct TestHas{
		pub m1:i32
	}
	#[derive(Debug,Clone, Copy)]
	struct TestM1<T>(pub PhantomData<T>);
	impl<T> Default for TestM1<T> {
		fn default() -> Self {
			Self(Default::default())
		}
	}
	#[derive(Debug,Clone, Copy)]
	struct TestM2<T>(pub PhantomData<T>);
	impl<T> Default for TestM2<T> {
		fn default() -> Self {
			Self(Default::default())
		}
	}
	impl<T> HasMarker for TestM1<T>
		where T:Deref<Target =  i32>
	{
		type Item = T;
	}
	impl<T> HasMarker for TestM2<T>
		where T:Deref<Target = bool>
	{
		type Item = T;
	}
	impl<'a> Has<TestM1<&'a i32>> for &'a TestHas {
	
		fn get_has(self,_marker:TestM1<&'a i32>)->&'a i32 {
			&self.m1
		}
	}
	impl<'a> Has<TestM2<Just<bool>>> for &'a TestHas {
	
		fn get_has(self,_marker:TestM2<Just<bool>>)->Just<bool> {
			Just(false)
		}
	}
	#[test]
	fn test(){
		
		let test_has=TestHas{
			m1:10
		};
		let testm1:TestM1<_>=TestM1::default();
		let testm2:TestM2<_>=TestM2::default();
		let testms: HCons<TestM1<_>, HCons<TestM2<_>, HNil>>=hlist![testm1,testm2];
		//let get_result=(&test_has).get_mult(testms);
		let r=HasMult::<HCons<TestM1<_>, HCons<TestM2<_>, HNil>>>::get_has_mult(&test_has, testms.clone());
		let r2=testms.map(hlist![
			|m|(&test_has).get_has(m),
			|m|(&test_has).get_has(m),
		]);
		println!("{:?}",r);
		println!("{:?}",r2);
	}
	#[test]
	fn test2(){
		fn testfn<T,R1,R2>(v:T)
			where T:HasMult<HList!(TestM1<R2>,TestM2<R2>)>
		{
			let testm1:TestM1<_>=TestM1::default();
			let testm2:TestM2<_>=TestM2::default();
			let r1=v.get_has(testm1);
		}
	}
}