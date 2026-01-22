use std::{array, ops::{ControlFlow, Deref, Index, Range}};

use crate::structures::step_iter::{SteppingIterator, SteppingIteratorFunction};

//use ndarray::Array1;



pub type NDimIndex<const DIM:usize>=[isize;DIM];

/// convert indexes with N dimensions in range and usize
pub struct NDimIndexer<const DIM:usize>{
    starts:[isize;DIM],
    steps:[usize;DIM],
    range_u:usize,
    lens:[Range<isize>;DIM],
}

pub trait TNDimIndexer<const DIM:usize> {
    fn length(&self) -> impl Deref<Target=usize>;
    fn lens(&self)->impl Deref<Target=[Range<isize>;DIM]>;
	fn steps(&self)->&[usize;DIM];
    // fn length(&self)->&Range<isize>;

    
    fn contains(&self,indexes:&NDimIndex<DIM>)->bool;

    fn contains_compressed(&self,index:usize)->bool;
    
    fn compress_index(&self,indexes:&NDimIndex<DIM>)->usize;
    fn decompress_index(&self,compressed_index:usize)->NDimIndex<DIM>;
	fn decompress_index_at_dim(&self,compressed_index:usize,dim:usize)->isize;
	fn add_index_at_dim(&self,compressed_index:usize,dim:usize,add_index:isize)->usize;
    fn iter<'a>(&'a self)->impl Iterator<Item=NDimIndex<DIM>> + 'a;
}

impl<const DIM:usize> NDimIndexer<DIM> {
    pub fn new_len(lens:[Range<isize>;DIM])->Self{
        let mut starts=[0isize;DIM];
        let mut steps=[0usize;DIM];
        let mut total_len:usize=1;
        for i in (0..DIM).rev() {
            starts[i]=lens[i].start;
            steps[i]=total_len;
            let dim_len= (lens[i].end-lens[i].start) as usize;
            total_len*=dim_len;
        }
        Self { starts, steps, range_u:total_len as usize, lens }
    }
    pub fn starts(&self)->&[isize;DIM]{&self.starts}
}

impl<const DIM:usize> TNDimIndexer<DIM> for NDimIndexer<DIM>{
    fn lens(&self)->impl Deref<Target = [std::ops::Range<isize>; DIM]>{&self.lens}
	fn steps(&self)->&[usize;DIM]{&self.steps}
    // fn length(&self)->&Range<isize>{&self.range}
    fn length(&self)->impl Deref<Target = usize>{&self.range_u}

    fn contains(&self,indexes:&NDimIndex<DIM>)->bool{
        //let mut pass=true;
        for i in 0..DIM {
            if self.lens[i].contains(&indexes[i]){
                //pass=true
            }else {
                return false;
            }
        }
        return true;
    }

    fn contains_compressed(&self,index:usize)->bool{
        index<self.range_u
    }
    
    fn compress_index(&self,indexes:&NDimIndex<DIM>)->usize{
        let mut res:usize=0;
        for i in 0..DIM {
            res+=(indexes[i]-self.lens[i].start) as usize * self.steps[i];
        }
        res
    }
    fn decompress_index(&self,mut compressed_index:usize)->NDimIndex<DIM> {
        array::from_fn(|i|{
            let step=self.steps[i];
            let (div,rem)=(compressed_index/step,compressed_index%step);
            compressed_index=div;
            rem as isize + self.lens[i].start
        })
    }
    fn iter<'a>(&'a self)->impl Iterator<Item = [isize; DIM]> + 'a {
        NDimIndexIter::<DIM, &[Range<isize>;DIM]>::new(&self.lens)
    }
	
	fn decompress_index_at_dim(&self,mut compressed_index:usize,dim:usize)->isize {
		if !(0..DIM).contains(&dim) {
			panic!("dim {0} out of range 0..{1}",dim,DIM);
		}
		let step=self.steps[dim];
		if dim>0 {
			let prev_dim=dim-1;
			let prev_step=self.steps[prev_dim];
			compressed_index=compressed_index%prev_step;
		}
		return (compressed_index/step) as isize + self.lens[dim].start;
	}
	
	fn add_index_at_dim(&self,compressed_index:usize,dim:usize,add_index:isize)->usize {
		if !(0..DIM).contains(&dim) {
			panic!("dim {0} out of range 0..{1}",dim,DIM);
		}
		let mut new_compressed=
			(compressed_index as isize)
			+ (self.steps[dim] as isize)*add_index
			;
		while new_compressed<0 {
			new_compressed+=self.range_u as isize;
		}
		while new_compressed>=self.range_u as isize {
			new_compressed-=self.range_u as isize;
		}
		return new_compressed as usize;
	}
}

// struct SteppingIndexIteratorFunction;

// impl<'a> SteppingIteratorFunction<&'a mut isize, &'a Range<isize>> for SteppingIndexIteratorFunction {
// 	fn step(&mut self, i:&mut isize, lens:&'a Range<isize>, _:usize)->ControlFlow<(),()> {
// 		*i+=1;
// 		if *i>=lens.end{
// 			*i=lens.start;
// 			return ControlFlow::Continue(());
// 		}else {
// 			return ControlFlow::Break(());
// 		}
// 	}
// }


// // fn index_loop(i:&mut isize,lens:&Range<isize>,_:usize)->ControlFlow<(),()> {
// // 	*i+=1;
// // 	if *i>=lens.end{
// // 		*i=lens.start;
// // 		return ControlFlow::Continue(());
// // 	}else {
// // 		return ControlFlow::Break(());
// // 	}
// // }

// type ANDimIndexIter<const DIM:usize>=SteppingIterator<DIM, NDimIndex<DIM>, [Range<isize>;DIM], SteppingIndexIteratorFunction>;


pub struct NDimIndexIter<const DIM:usize,Lens> {
    lens:Lens,
    cur:[isize;DIM],
    ended:bool
}

impl<const DIM:usize,Lens,LensDeref> NDimIndexIter<DIM,Lens> 
	where Lens:Deref<Target = LensDeref>,
		LensDeref:Index<usize,Output = Range<isize>>,
		for<'a>&'a LensDeref: IntoIterator<Item = &'a Range<isize>,IntoIter : DoubleEndedIterator+ExactSizeIterator>,
{
    pub fn new(lens:Lens)->Self{Self{cur:array::from_fn(|i|(*lens)[i].start),lens,ended:false}}
}

impl<const DIM:usize,Lens,LensDeref>  Iterator for NDimIndexIter<DIM,Lens> 
	where Lens:Deref<Target = LensDeref>,
		LensDeref:Index<usize,Output = Range<isize>>,
		for<'a>&'a LensDeref: IntoIterator<Item = &'a Range<isize>,IntoIter : DoubleEndedIterator+ExactSizeIterator>,

{
    type Item=NDimIndex<DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended{
            return None;
        }
        let res=self.cur.clone();
        let cur=&mut self.cur;
        let lens=&self.lens;
        let iterate=cur.iter_mut().zip((&lens).into_iter()).rev().try_for_each(|(c,l)|{
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


#[test]
fn test() {
    let a_ndidxer=NDimIndexer::new_len([-5..5,-5..5,-5..5]);
    let a_ndidx=[-2,0,2];
    let a_cidx=a_ndidxer.compress_index(&a_ndidx);
    println!("{:?}",a_ndidxer.starts());
    println!("{:?}",a_ndidxer.steps());
    println!("{:?}",a_cidx);
    let a_scidx=a_ndidxer.decompress_index(a_cidx);
    println!("{:?}",a_scidx);
    
    // for b_compress_index in b_compress_index_iter {
    //     println!("{:?}",b_compress_index)
    // }
    
}

#[test]
fn test_iterate(){
    let indexer=NDimIndexer::new_len([0..2,0..3,0..4]);
    for i in indexer.iter() {
        println!("{:?} : {}",i,indexer.compress_index(&i));
    }
}