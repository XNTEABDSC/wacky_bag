use std::{array, ops::{ControlFlow, Deref, Range}};

use crate::structures::n_dim_array::n_dim_index::NDimIndex;


pub struct NDimIndexIter<const DIM:usize,Lens> {
    lens:Lens,
    cur:[isize;DIM],
    ended:bool
}

impl<const DIM:usize,Lens> NDimIndexIter<DIM,Lens> 
	where Lens:Deref<Target = [Range<isize>;DIM]>,
{
    pub fn new(lens:Lens)->Self{Self{cur:array::from_fn(|i|(*lens)[i].start),lens,ended:false}}
}

impl<const DIM:usize,Lens> Iterator for NDimIndexIter<DIM,Lens> 
	where Lens:Deref<Target = [Range<isize>;DIM]>,

{
    type Item=NDimIndex<DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended{
            return None;
        }
        let res=self.cur.clone();
        let cur=&mut self.cur;
        let lens=self.lens.deref();
        let iterate=cur.iter_mut().zip(lens.into_iter()).rev().try_for_each(|(c,l)|{
            *c+=1;
            if *c>=l.end{
                *c=l.start;
                return ControlFlow::Continue(());
            }else {
                return ControlFlow::Break(());
            }
        });
        if iterate.is_continue() {
            self.ended=true;
        }
        return Some(res);
    }
}