//! [`GridIter`]

use std::sync::{LazyLock, RwLock};
use std::{collections::BinaryHeap, usize};
//use lazy_static::lazy_static;

/// Enum grid position, by the order of the distance of grid's (i-0.5,j-0.5,1,1) closest point to (0,0)
/// 0<=y<=x
/// includes (0,0)
/// automatically grow
/// safe for multiple iter itering parallelly
pub struct GridIter{
    index:usize
}
// struct ToOct{

// }

type Point=(i32,i32);
/// point and distance
#[allow(missing_docs)]
pub struct PointAndDistance{
    pub point:Point,
    pub distance:f32,
    pub distancesq:f32,
}

impl PartialEq for PointAndDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for PointAndDistance {
    
}

impl PartialOrd for PointAndDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for PointAndDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.distance.partial_cmp(&self.distance).unwrap();
    }
}

struct GridIterData{
    pub grid_iter_checked_len:i32,
    pub grid_iter_checked_cache:Vec<PointAndDistance>,
    pub grid_iter_unchecked_len:BinaryHeap<PointAndDistance>,
}

static GRID_ITER_UNCHECKED_LEN_MUTEX: LazyLock<RwLock<GridIterData>> =LazyLock::new(||{
    RwLock::new( 
        GridIterData{
            grid_iter_unchecked_len:BinaryHeap::<PointAndDistance>::new(),
            grid_iter_checked_len:-1,
            grid_iter_checked_cache:Vec::<PointAndDistance>::new()
        }
    )
});

// lazy_static!{
//     static ref GRID_ITER_UNCHECKED_LEN_MUTEX: Mutex<GridIterData> =
//     Mutex::new( 
//         GridIterData{
//             grid_iter_unchecked_len:BinaryHeap::<PointAndDistance>::new(),
//             grid_iter_checked_len:-1,
//             grid_iter_checked_cache:Vec::<PointAndDistance>::new()
//         }
//     );
// }

fn grow(needed:usize){
    {
        let mut dwadwad=GRID_ITER_UNCHECKED_LEN_MUTEX.write().unwrap();
        
        //let mut grid_iter_unchecked_len=&mut dwadwad.grid_iter_unchecked_len;
        //let mut grid_iter_checked_cache=&mut dwadwad.grid_iter_checked_cache;
        //let mut grid_iter_checked_len=&mut dwadwad.grid_iter_checked_len;
        if needed>=dwadwad.grid_iter_checked_cache.len(){
            fn get_lensq(p:&Point)->f32{
                let x:f32=match p.0 {
                    0=>0f32,
                    n=>n as f32-0.5f32
                };
                let y:f32=match p.1 {
                    0=>0f32,
                    n=>n as f32-0.5f32
                };
                return x*x+y*y;
            }
            let new_grid_iter_checked_len=dwadwad.grid_iter_checked_len+1;
            let mut y=0;
            while y<=new_grid_iter_checked_len {
                let p=(new_grid_iter_checked_len,y);
                let distsq=get_lensq(&p);
                let dist=distsq.sqrt();
                dwadwad.grid_iter_unchecked_len.push(PointAndDistance{
                    point:p,
                    distancesq:distsq,
                    distance:dist
                });
                y+=1;
            }
            let grid_iter_checked_len_f32=new_grid_iter_checked_len as f32;
            loop {
                let v=dwadwad.grid_iter_unchecked_len.peek();
                match v {
                    None=>break,
                    Some(x)=>{
                        if x.distance<=grid_iter_checked_len_f32 {
                            let new_value=dwadwad.grid_iter_unchecked_len.pop().unwrap();
                            dwadwad.grid_iter_checked_cache.push(new_value);
                        }else{
                            break;
                        }
                    }
                }
            }
            dwadwad.grid_iter_checked_len=new_grid_iter_checked_len;
        }
    }
}
impl GridIter {
	/// new
    pub fn new()->Self{Self{index: 0}}
}
impl Iterator for GridIter {
    type Item=&'static PointAndDistance;

    fn next(&mut self) -> Option<Self::Item> {
        let i=self.index;
        self.index=self.index+1;
        let mut lock=GRID_ITER_UNCHECKED_LEN_MUTEX.read().unwrap();

        if i>=lock.grid_iter_checked_cache.len()
        {
            drop(lock);
            grow(i);
			lock=GRID_ITER_UNCHECKED_LEN_MUTEX.read().unwrap();
        }
		
        let awd=lock.grid_iter_checked_cache.as_ptr();
        drop(lock);
        return unsafe {
             awd.add(i).as_ref()
        };
        
    }
}
#[test]
fn test_grid_iter_thread(){
    use std::thread;
    let mut handles=Vec::new();
    for i in 10..100 {
        handles.push(thread::spawn(move ||{
            let mut count=0;
            for p in GridIter::new() {
                count+=1;
                if count>=i {
                    break;
                }
                if i%10==0{
                    println!("{},{}",p.point.0,p.point.1)
                }
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
#[test]
fn test_grid_iter(){

    let mut count=0;
    for p in GridIter::new() {
        count+=1;
        if count>=100 {
            break;
        }
        println!("{},{}",p.point.0,p.point.1)
    }
}