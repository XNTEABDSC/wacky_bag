//! [loop_wrap_angle_assign]
//! [loop_wrap_angle]

use simba::scalar::RealField;

/// Limit the angle in `-Num::pi().. Num::pi()`, in place.
pub fn loop_wrap_angle_assign<Num:RealField+num_traits::Num+Ord+Copy>(angle:&mut Num){
    crate::utils::loop_wrap::loop_wrap_assign(angle, &(-Num::pi().. Num::pi()), Num::pi()*(Num::two_pi()));
}

/// Limit the angle in `-Num::pi().. Num::pi()`.
pub fn loop_wrap_angle<Num:RealField+num_traits::Num+Ord+Copy>(angle:Num)->Num{
    crate::utils::loop_wrap::loop_wrap(angle, &(-Num::pi()..Num::pi()), Num::two_pi())
}