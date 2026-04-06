// use crate::num::Num;

// use num_traits::Num;
use simba::scalar::RealField;

pub fn normal_pdf<Num>(x:Num)->Num
	where Num:RealField+num_traits::Num+Copy
{
	//Num::FRAC_1_SQRT_2PI
    return Num::frac_2_sqrt_pi() /(Num::one()+Num::one())  * Num::exp(-x/ (Num::one()+Num::one()) *x)
}