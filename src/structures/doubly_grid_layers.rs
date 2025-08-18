use std::ops::Range;

use crate::structures::mvec::MVec;


pub struct DoublyGrid2DLayers<T>{
    pub values:Vec<Vec<Vec<T>>>
}

impl<T> DoublyGrid2DLayers<T> {
    pub fn new(width:i32,height:i32,mut genfn:impl FnMut(i32,i32,i32)->T)->Self{
        let mut grid_len=1;
        let mut values:Vec<Vec<Vec<T>>>=Vec::new();
        loop {
            
            let mut gridy:Vec<Vec<T>>=Vec::new();
            let mut y=0;
            let mut ymax=height/grid_len;
            let mut xmax=width/grid_len;

            if ymax*grid_len<height{ymax=ymax+1}
            if xmax*grid_len<width{xmax=xmax+1}

            while y<ymax {
                
                let mut gridx:Vec<T>=Vec::new();
                let mut x = 0;
                while x<xmax {
                    gridx.push(genfn(grid_len,y,x));
                    x+=1;
                }
                gridy.push(gridx);
                y+=1;
            }
            values.push(gridy);

            if ymax==1&&xmax==1{
                break;
            }
            grid_len=grid_len*2;
        }
        return Self{values};
    }
}

pub fn doubly_grid2d_layers_interacts(target_layer:i32,target_x:i32,target_y:i32,current_layer:i32)->MVec<Range<i32>,2>{
    let layer_dif=current_layer-target_layer;
    let target_x2=target_x+1;
    let target_y2=target_y+1;
    
    let new_x=target_x>>layer_dif;
    let new_y=target_y>>layer_dif;
    if layer_dif>=0{
        let new_x2=new_x+1;
        let new_y2=new_y+1;
        return MVec::new(Range { start: new_x, end: new_x2 }, Range { start: new_y, end: new_y2 });

    }else {
        let new_x2=target_x2>>layer_dif;
        let new_y2=target_y2>>layer_dif;
        return MVec::new(Range { start: new_x, end: new_x2 }, Range { start: new_y, end: new_y2 });
    }
}

