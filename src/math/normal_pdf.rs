//! [normal_pdf]

use simba::scalar::RealField;
/// `Num::frac_2_sqrt_pi() /(Num::one()+Num::one())  * Num::exp(-x/ (Num::one()+Num::one()) *x)`
pub fn normal_pdf<Num>(x:Num)->Num
	where Num:RealField+num_traits::Num+Copy
{
	//Num::FRAC_1_SQRT_2PI
	let two=Num::one()+Num::one();
	let tow_recip=two.recip();
    return Num::frac_2_sqrt_pi() * tow_recip  * Num::exp(-x * tow_recip *x)
}