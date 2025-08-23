use std::{array, ops::{Range, RangeInclusive}};

use crate::{structures::{doubly_slice_layers::{doubly_slice_layers_overlap, find_range_at_doubly_slice_i, DoublySliceIndex}, mvec::MVec, n_dim_index::NDimIndexer, n_dim_vec::NDimArray}, utils::range_inclusive_upper_convert::range_inclusive_convert_cover};
pub struct DoublyGridIndex<const DIM:usize>{
    pub layer:isize,pub pos:MVec<isize,DIM>
}
pub type DoublyGrid2DIndex=DoublyGridIndex<2>;

pub struct DoublyGridLayers<const DIM:usize,T>{
    pub values:Vec<NDimArray<DIM,T>>
}

impl<T,const DIM:usize> DoublyGridLayers<DIM,T> {
    pub fn new(mut lens:[RangeInclusive<isize>;DIM],mut genfn:impl FnMut(DoublyGridIndex<DIM>)->T)->Self {
        let mut values:Vec<NDimArray<DIM,T>>=Vec::new();
        let mut layer_index=0;
        
        loop {
            let indexer=NDimIndexer::new_len(lens.clone().map(|v|{
                let inner=v.into_inner();
                inner.0..(inner.1+1)
            }));
            let layer_values=NDimArray::from_fn(indexer, |index|genfn(DoublyGridIndex { layer: layer_index, pos: index }));
            values.push(layer_values);
            if lens.iter().all(|v|(v.end()-v.start()+1)<=1) {
                break;
            }
            layer_index+=1;
            lens=lens.map(|v|{
                let (l,r)=v.clone().into_inner();
                (l.div_euclid(2))..=(r.div_euclid(2)+r.rem_euclid(2))
            })
        }
        Self { values: values }
    }

    pub fn get_doubly_grid_index(&self,index:DoublyGridIndex<DIM>)->Option<&T> {
        self.values.get(index.layer as usize).map_or(None, 
            |values| values.get(index.pos))
    }
    pub fn get_mut_doubly_grid_index(&mut self,index:DoublyGridIndex<DIM>)->Option<&mut T> {
        self.values.get_mut(index.layer as usize).map_or(None, 
            |values| values.get_mut(index.pos))
    }
}



pub fn find_doubly_grid_layers_overlap<const DIM:usize>(target:DoublyGridIndex<DIM>,current_layer:isize)->MVec<Range<isize>,DIM>{
    //target_layer:i32,target_x:i32,target_y:i32
    return MVec(array::from_fn(|i|doubly_slice_layers_overlap(DoublySliceIndex{layer:target.layer,pos:target.pos[i]}, current_layer)));
}


/// find the minimun `DoublyGridIndex` which covers `rect`
pub fn find_rect_object_at_doubly_grid_layers<const DIM:usize,Num>(rect:MVec<RangeInclusive<Num>,DIM>)->DoublyGridIndex<DIM>
    where Num:Into<isize>
{
    find_rect_object_at_doubly_grid_layers_i::<DIM>(MVec(
        rect.0.map(|v|{
            
            range_inclusive_convert_cover(v)
        })
    ))
}

/// for i32, find the minimun `DoublyGridIndex` which covers `rect`
pub fn find_rect_object_at_doubly_grid_layers_i<const DIM:usize>(rect:MVec<RangeInclusive<isize>,DIM>)->DoublyGridIndex<DIM>
{
    let slices: [DoublySliceIndex; DIM]=rect.0.map(|range|{find_range_at_doubly_slice_i(range)});// array::from_fn(|i|find_range_at_doubly_slice_i32(rect[i]));
    let mut max_layer=0;
    slices.iter().for_each(|v|{if v.layer>max_layer{max_layer=v.layer}});
    return DoublyGridIndex{
        layer:max_layer,
        pos:MVec(array::from_fn(|i|{
            doubly_slice_layers_overlap(slices[i], max_layer).start
        }))
    };
}