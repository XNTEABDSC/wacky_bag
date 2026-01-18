use std::ops::{AddAssign, RangeBounds, SubAssign};

pub fn loop_wrap_assign<T,TRange,T2>(value:&mut T,range:&TRange,len:T2)->isize
    where T:AddAssign<T2>+SubAssign<T2>+Ord,
	TRange:RangeBounds<T>,
	T2:Copy
{
	let mut i=0;
	match range.start_bound() {
		std::ops::Bound::Included(l) =>{
			while *value<*l {
				*value+=len;
				i-=1;
			}
		},
		std::ops::Bound::Excluded(l) =>{
			while *value<=*l {
				*value+=len;
				i-=1;
			}
		},
		std::ops::Bound::Unbounded => {},
	}

	match range.end_bound() {
		std::ops::Bound::Included(r) =>{
			while *r<*value {
				*value-=len;
				i+=1;
			}
		},
		std::ops::Bound::Excluded(r) =>{
			while *r<=*value {
				*value-=len;
				i+=1;
			}
		},
		std::ops::Bound::Unbounded => {},
	}
	return i;
}

pub fn loop_wrap<T,TRange,T2>(mut value:T,range:&TRange,len:T2)->T
    where T:AddAssign<T2>+SubAssign<T2>+Ord,TRange:RangeBounds<T>,T2:Copy
{
    loop_wrap_assign(&mut value,range,len);
    return value;
}
