use std::ops::{RangeInclusive};

/// convert \[l:`Num`,r:`Num`\] to \[li:`isize`,ri:`isize`\], all slices \[i,i+1) in \[li,ri\] covers \[l,r\]
pub fn range_inclusive_convert_cover<Num>(rangei:RangeInclusive<Num>)->RangeInclusive<isize>
    where Num:Into<isize>
{
    
    let inner=rangei.into_inner();
    (inner.0.into())..=
    (inner.1.into()+1)
}