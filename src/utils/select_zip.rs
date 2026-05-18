//! [`HSelectZippable`]

use frunk::{HCons, HNil, Poly};

use crate::utils::type_fn::TypeFunc;


/// select b with [TypeFunc], and zip
pub trait HSelectZippable<TypeFunc,B>
{
	/// the result
	type Output;
	/// select b with tf, and zip
	fn select_zip(self,tf:TypeFunc,b:B)->Self::Output;
}

impl<TF> HSelectZippable<TF,HNil> for HNil {
	type Output=HNil;

	fn select_zip(self,_tf:TF,_b:HNil)->Self::Output {
		HNil
	}
}

impl<TF,TInputH,TInputT,BInputH,BInputT> HSelectZippable<Poly<TF>,HCons<BInputH,BInputT>> for HCons<TInputH,TInputT>
	where TF:TypeFunc<TInputH,Output = BInputH>,
		TInputT:HSelectZippable<Poly<TF>,BInputT>
{
	type Output=HCons<(TInputH,BInputH),TInputT::Output>;

	fn select_zip(self,tf:Poly<TF>,b:HCons<BInputH,BInputT>)->Self::Output {
		HCons{head:(self.head,b.head),tail:self.tail.select_zip(tf, b.tail)}
	}
}