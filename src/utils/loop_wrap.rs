use std::ops::{AddAssign, SubAssign};

pub fn loop_wrap_assign<'a,T,T2>(value:&mut T,left:T,right:T,len:&'a T2)
    where T:AddAssign<&'a T2>+SubAssign<&'a T2>+Ord,
{
    while *value<left {
        *value+=len;
    }
    while *value>right {
        *value-=len;
    }
}

pub fn loop_wrap<'a,T,T2>(mut value:T,left:T,right:T,len:&'a T2)->T
    where T:AddAssign<&'a T2>+SubAssign<&'a T2>+Ord+'a,
{
    loop_wrap_assign(&mut value,left,right,&len);
    return value;
}