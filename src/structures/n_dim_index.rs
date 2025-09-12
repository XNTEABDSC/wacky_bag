use std::{array, ops::Range};

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
        let mut i=0;
        loop {
            cur[i]+=1;
            if cur[i]>=lens[i].end{
                cur[i]=lens[i].start;
                i+=1;
                if i>=lens.len(){
                    self.ended=true;
                    break;
                }
            }else {
                break;
            }
        }
        return Some(res);
    }
}

impl<const DIM:usize> NDimIndexer<DIM>{
    pub fn lens(&self)->&[Range<isize>;DIM]{&self.lens}
    pub fn starts(&self)->&[isize;DIM]{&self.starts}
    pub fn steps(&self)->&[isize;DIM]{&self.steps}
    pub fn length(&self)->&Range<isize>{&self.range}

    pub fn new_len(lens:[Range<isize>;DIM])->Self{
        //let mut last_step:isize=1;
        //let mut last_base:isize=0;
        //let mut last_count=0;


        let mut last_range:Range<isize>=0..1;

        let mut starts=[0;DIM];
        let mut steps=[0;DIM];

        for i in 0..DIM {
            
            let step=last_range.end-last_range.start;
            let start=step*lens[i].start;
            let count=lens[i].end-lens[i].start;
            let end=start+step*count;
            last_range=start..end;
            starts[i]=start;
            steps[i]=step;
        }

        /*
        let calc:[(isize,isize);N]=array::from_fn(|i|{
            let step=last_range.end-last_range.start;
            let start=step*lens[i].start;
            let count=lens[i].end-lens[i].start;
            let end=start+step*count;
            last_range=start..end;
            (start,step)
        });

        let (starts,steps)=(array::from_fn(|i|calc[i].0),array::from_fn(|i|calc[i].1));
        */

        Self{starts,steps,range:last_range.clone(),lens,range_u:(last_range.end-last_range.start) as usize}
    }
    
    pub fn contains(&self,indexes:NDimIndex<DIM>)->bool{
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

    pub fn contains_compressed_u(&self,index:usize)->bool{
        0<=index&&index<self.range_u
    }
    
    pub fn compress_index_u(&self,indexes:NDimIndex<DIM>)->usize{
        let mut res:usize=0;
        for i in 0..DIM {
            res+=((indexes[i]-self.lens[i].start)*self.steps[i]) as usize;
        }
        res
    }
    pub fn decompress_index_u(&self,mut compressed_index:usize)->NDimIndex<DIM> {
        let mut res=[0;DIM];
        let mut i=DIM-1;
        loop{
            let step=self.steps[i] as usize;
            let (div,rem)=(compressed_index/step,compressed_index%step);
            res[i]=div as isize+self.lens[i].start;
            compressed_index=rem;
            if i==0{
                break;
            }
            i-=1;
        }
        res
    }
    pub fn iter<'a>(&'a self)->NDimIndexIter<'a,DIM> {
        NDimIndexIter::new(&self.lens)
    }
    /*
    pub fn compress_index_i(&self,indexes:NDimIndexI<N>)->isize{
        let mut res=0;
        for i in 0..N {
            res+=(indexes[i])*self.steps[i]
        }
        res
    }
    pub fn seperate_index_i(&self,mut compressed_index:isize)->NDimIndexI<N>{
        let mut res=[0;N];
        let mut i=N-1;
        loop{
            todo!();
            let compressed_index_shift=compressed_index-self.starts[i];
            
            let (div,rem)=(compressed_index_shift/self.steps[i],compressed_index_shift%self.steps[i]);
            res[i]=div;
            compressed_index=rem;
            if i==0{
                break;
            }
            i-=1;
        }
        MVec(res)
    } */
}

#[test]
fn test() {
    let a_ndidxer=NDimIndexer::new_len([-5..5,-5..5,-5..5]);
    let a_ndidx=[-2,0,2];
    let a_cidx=a_ndidxer.compress_index_u(a_ndidx);
    println!("{:?}",a_ndidxer.starts());
    println!("{:?}",a_ndidxer.steps());
    println!("{:?}",a_cidx);
    let a_scidx=a_ndidxer.decompress_index_u(a_cidx);
    println!("{:?}",a_scidx);
    
    // for b_compress_index in b_compress_index_iter {
    //     println!("{:?}",b_compress_index)
    // }
    
}