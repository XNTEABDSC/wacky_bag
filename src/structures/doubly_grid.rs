use std::ops::Range;

use crate::structures::mvec::MVec;


pub struct DoublyGrid2d<T>{
    pub values:Vec<Vec<Vec<T>>>
}

impl<T> DoublyGrid2d<T> {
    pub fn new(width:usize,height:usize,mut genfn:impl FnMut(usize,usize,usize)->T)->Self{
        let mut grid_len=1;
        let values:Vec<Vec<Vec<T>>>=Vec::new();
        while true {
            
            let mut gridy:Vec<Vec<T>>=Vec::new();
            let mut y=0;
            let ymax=height/grid_len;
            let xmax=width/grid_len;

            if (ymax*grid_len<height){ymax=ymax+1}
            if (xmax*grid_len<width){xmax=xmax+1}

            while y<ymax {
                
                let mut gridx:Vec<T>=Vec::new();
                let mut x = 0;
                while x<xmax {
                    gridx.append(genfn(grid_len,y,x));
                    x+=1;
                }
                gridy.append(gridx);
                y+=1;
            }
            values.append(gridy);

            if ymax==1&&xmax==1{
                break;
            }
            grid_len=grid_len*2;
        }
    }
}


fn doubly_grid2d_interacts(target_layer:usize,target_x:usize,target_y:usize,max_layer:usize,current_layer:usize)->MVec<Range<usize>,2>{
    let layer_dif=current_layer-target_layer;
    // if layer_dif==0{
    //     return MVec::new(target_x..=target_x, target_y..=target_y);
    // }
    if current_layer>=max_layer{
        panic!("current_layer>max_layer")
    }
    if layer_dif>=0{
        let new_x=target_x>>layer_dif;
        let new_y=target_y>>layer_dif;
        return MVec::new(new_x..=new_x, new_y..=new_y);
    }else {
        
    }
}
