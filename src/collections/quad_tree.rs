use std::collections::btree_map::Values;

use crate::structures::grid::Grid2D;

pub struct QuadTree<TValue>{
    values:Vec<Grid2D<Vec<Vec<TValue>>>>
}

impl<TValue> QuadTree<TValue>{
    pub fn new<TFn:FnMut(usize,usize,usize)->TValue>(depth:usize,mut init_value_fn:TFn)->Self {
        let mut values=Vec::with_capacity(depth);
        for layer in 0..depth {
            let len=1<<layer;
            let mut values_layer_y=Vec::with_capacity(len);
            for y in 0..len{
                let mut values_layer_x=Vec::with_capacity(len);
                for x in 0..len{
                    values_layer_x.push(init_value_fn(layer,x,y));
                }
                values_layer_y.push(values_layer_x);
            }
            values.push(Grid2D { values: values_layer_y });
        }
        return Self{values};
    }

    pub fn get(&self,layer:usize)->&Grid2D<Vec<Vec<TValue>>> {
        &self.values[layer]
    }
    pub fn get_mut(&mut self,layer:usize)->&mut Grid2D<Vec<Vec<TValue>>> {
        &mut self.values[layer]
    }

}