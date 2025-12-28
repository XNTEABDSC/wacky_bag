use std::{array, ops::{ControlFlow, Range}};

//use ndarray::Array1;



pub type NDimIndex<const DIM:usize>=[isize;DIM];

/// convert indexes with N dimensions in range and usize
pub struct NDimIndexer<const DIM:usize>{
    starts:[isize;DIM],
    steps:[isize;DIM],
    range:Range<isize>,
    range_u:usize,
    lens:[Range<isize>;DIM],
}

pub struct NDimIndexIter<'a,const DIM:usize>{
    lens:&'a [Range<isize>;DIM],
    cur:[isize;DIM],
    ended:bool
}

pub trait TNDimIndex<const DIM:usize> {
    fn length_u(&self) -> &usize;
    fn lens(&self)->&[Range<isize>;DIM];
    fn length(&self)->&Range<isize>;

    
    fn contains(&self,indexes:&NDimIndex<DIM>)->bool;

    fn contains_compressed_u(&self,index:usize)->bool;
    
    fn compress_index_u(&self,indexes:&NDimIndex<DIM>)->usize;
    fn decompress_index_u(&self,compressed_index:usize)->NDimIndex<DIM>;
    fn iter<'a>(&'a self)->NDimIndexIter<'a,DIM>;
}

impl<const DIM:usize> NDimIndexer<DIM> {
    pub fn new_len(lens:[Range<isize>;DIM])->Self{
        let mut starts=[0isize;DIM];
        let mut steps=[0isize;DIM];
        let mut total_len:isize=1;
        for i in (0..DIM).rev() {
            starts[i]=lens[i].start;
            steps[i]=total_len;
            let dim_len=lens[i].end-lens[i].start;
            total_len*=dim_len;
        }
        Self { starts, steps, range:0..total_len, range_u:total_len as usize, lens }
    }
    pub fn starts(&self)->&[isize;DIM]{&self.starts}
    pub fn steps(&self)->&[isize;DIM]{&self.steps}
}

impl<const DIM:usize> TNDimIndex<DIM> for NDimIndexer<DIM>{
    fn lens(&self)->&[Range<isize>;DIM]{&self.lens}
    fn length(&self)->&Range<isize>{&self.range}
    fn length_u(&self)->&usize{&self.range_u}

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

    fn contains_compressed_u(&self,index:usize)->bool{
        index<self.range_u
    }
    
    fn compress_index_u(&self,indexes:&NDimIndex<DIM>)->usize{
        let mut res:usize=0;
        for i in 0..DIM {
            res+=((indexes[i]-self.lens[i].start)*self.steps[i]) as usize;
        }
        res
    }
    fn decompress_index_u(&self,mut compressed_index:usize)->NDimIndex<DIM> {
        array::from_fn(|i|{
            let step=self.steps[i] as usize;
            let (div,rem)=(compressed_index/step,compressed_index%step);
            compressed_index=div;
            rem as isize + self.lens[i].start
        })
    }
    fn iter<'a>(&'a self)->NDimIndexIter<'a,DIM> {
        NDimIndexIter::new(&self.lens)
    }
}


impl<'a,const DIM:usize> NDimIndexIter<'a,DIM> {
    pub fn new(lens:&'a [Range<isize>;DIM])->Self{Self{lens,cur:array::from_fn(|i|lens[i].start),ended:false}}
}

impl<'a,const DIM:usize> Iterator for NDimIndexIter<'a,DIM> {
    type Item=NDimIndex<DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended{
            return None;
        }
        let res=self.cur.clone();
        let cur=&mut self.cur;
        let lens=&self.lens;
        let iterate=cur.iter_mut().zip(lens.iter()).rev().try_for_each(|(c,l)|{
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
        // cur.iter_mut().rev().for_each(|c,i|{
        //     *c+=1;
        //     if *c>=lens[i].end{
        //         *c=lens[i].start;
        //     }else {
        //         return;
        //     }
        // });
        // let mut i=DIM;
        // loop {
        //     i-=1;
        //     cur[i]+=1;
        //     if cur[i]>=lens[i].end{
        //         cur[i]=lens[i].start;
        //         if i==0{
        //             self.ended=true;
        //             break;
        //         }
        //     }else {
        //         break;
        //     }
        // }
        // let mut i=0;
        // loop {
        //     cur[i]+=1;
        //     if cur[i]>=lens[i].end{
        //         cur[i]=lens[i].start;
        //         i+=1;
        //         if i>=lens.len(){
        //             self.ended=true;
        //             break;
        //         }
        //     }else {
        //         break;
        //     }
        // }
        return Some(res);
    }
}


#[test]
fn test() {
    let a_ndidxer=NDimIndexer::new_len([-5..5,-5..5,-5..5]);
    let a_ndidx=[-2,0,2];
    let a_cidx=a_ndidxer.compress_index_u(&a_ndidx);
    println!("{:?}",a_ndidxer.starts());
    println!("{:?}",a_ndidxer.steps());
    println!("{:?}",a_cidx);
    let a_scidx=a_ndidxer.decompress_index_u(a_cidx);
    println!("{:?}",a_scidx);
    
    // for b_compress_index in b_compress_index_iter {
    //     println!("{:?}",b_compress_index)
    // }
    
}

#[test]
fn test_iterate(){
    let indexer=NDimIndexer::new_len([0..2,0..3,0..4]);
    for i in indexer.iter() {
        println!("{:?} : {}",i,indexer.compress_index_u(&i));
    }
}