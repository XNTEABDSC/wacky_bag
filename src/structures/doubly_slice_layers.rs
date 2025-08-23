use std::ops::{Range, RangeInclusive};

use crate::utils::range_inclusive_upper_convert::range_inclusive_convert_cover;

#[derive(Clone, Copy, PartialEq, Eq,Debug)]
pub struct DoublySliceIndex{
    pub layer:isize,pub pos:isize
}

impl DoublySliceIndex {
    /// get `range` of `DoublySliceIndex`
    pub fn into_range(self)->Range<isize> {
        let layer_dist=1<<self.layer;
        let start=self.pos*layer_dist;
        let end=start+layer_dist;
        return start..end;
    }
    /// check whether `self` covers `range`
    pub fn range_at_doubly_slice_i(self,range:RangeInclusive<isize>)->bool{
        let layer_dist=1<<self.layer;
        let slice_start=self.pos*layer_dist;
        let slice_end=slice_start+layer_dist;
        return slice_start<=*range.start()&&*range.end()<slice_end;
    }
}

impl From<DoublySliceIndex> for Range<isize> {
    fn from(value: DoublySliceIndex) -> Self {
        value.into_range()
    }
}


/// for `i32`, find the minimun `slice` which covers `range`
pub fn find_range_at_doubly_slice_i(range:RangeInclusive<isize>)->DoublySliceIndex{
    let (x0,x1)=range.into_inner();
    
    let dist=x1-x0+1;

    let mut layer=0;
    while 1<<layer<dist {
        layer+=1;
    }
    loop{
        let x0_sh=x0>>layer;
        let x1_sh=x1>>layer;
        if x1_sh-x0_sh==0{
            return DoublySliceIndex{
                layer,pos:x0_sh
            };
        }
        layer+=1;
    }
}

/// find the minimun `slice` which covers `range`
pub fn find_range_at_doubly_slice<Num>(range:RangeInclusive<Num>)->DoublySliceIndex
    where Num:Into<isize>
{
    find_range_at_doubly_slice_i(range_inclusive_convert_cover(range))
}
#[test]
fn test(){
    println!("{:?}",find_range_at_doubly_slice_i(10..=15));
    println!("{:?}",find_range_at_doubly_slice_i(10..=11));
    println!("{:?}",find_range_at_doubly_slice_i(10..=10));
    //assert_eq!(range_at_doubly_slice_i32(10..=15))
}

/// find the range of slices at `current_layer` which overlap with `target`
pub fn doubly_slice_layers_overlap(target:DoublySliceIndex,current_layer:isize)->Range<isize>{
    //target_layer:i32,target_x:i32,target_y:i32
    let (target_layer,target_pos)=(target.layer,target.pos);
    let layer_dif=current_layer-target_layer;
    let target_pos2=target_pos+1;
    
    let new_target_pos=target_pos>>layer_dif;
    let new_target_pos2=if layer_dif>=0{
        new_target_pos+1
    }else{
        target_pos2>>layer_dif
    };
    return new_target_pos..new_target_pos2
}

