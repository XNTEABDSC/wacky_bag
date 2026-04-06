// use crate::num::Num;

use simba::scalar::RealField;

pub fn loop_wrap_angle_assign<Num:RealField+num_traits::Num+Ord+Copy>(angle:&mut Num){
    crate::utils::loop_wrap::loop_wrap_assign(angle, &(-Num::pi().. Num::pi()), Num::pi()*(Num::two_pi()));
}

pub fn loop_wrap_angle<Num:RealField+num_traits::Num+Ord+Copy>(angle:Num)->Num{
    crate::utils::loop_wrap::loop_wrap(angle, &(-Num::pi()..Num::pi()), Num::two_pi())
}