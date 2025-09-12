use std::array;



pub type NDimIndexU<const DIM:usize>=[usize;DIM];
pub struct NDimIndexerU<const DIM:usize>{
    steps:[usize;DIM],
    length:usize,
    lens:[usize;DIM],
}

pub struct NDimIndexUIter<'a,const DIM:usize>{
    lens:&'a [usize;DIM],
    cur:[usize;DIM],
    ended:bool
}

impl<'a,const DIM:usize> NDimIndexUIter<'a,DIM> {
    pub fn new(lens:&'a [usize;DIM])->Self{Self{lens,cur:[0;DIM],ended:false}}
}

impl<'a,const DIM:usize> Iterator for NDimIndexUIter<'a,DIM> {
    type Item=NDimIndexU<DIM>;

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
            if cur[i]>=lens[i]{
                cur[i]=0;
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

impl<const DIM:usize> NDimIndexerU<DIM>{
    pub fn lens(&self)->&[usize;DIM]{&self.lens}
    pub fn bases(&self)->&[usize;DIM]{&self.steps}
    pub fn length(&self)->usize{self.length}

    pub fn new_len(lens:[usize;DIM])->Self{
        let mut cur=1;
        let base=array::from_fn(|i|{
            let res=cur;
            cur=cur*lens[i];
            res
        });
        Self{steps: base,length:cur,lens}
    }
    pub fn compress_index(&self,indexes:NDimIndexU<DIM>)->usize{
        let mut res=0;
        for i in 0..DIM {
            res+=indexes[i]*self.steps[i]
        }
        res
    }
    pub fn seperate_index(&self,mut compressed_index:usize)->NDimIndexU<DIM>{
        let mut res=[0;DIM];
        let mut i=DIM-1;
        loop{
            let (div,rem)=(compressed_index/self.steps[i],compressed_index%self.steps[i]);
            res[i]=div;
            compressed_index=rem;
            if i==0{
                break;
            }
            i-=1;
        }
        res
    }
    pub fn iter<'a>(&'a self)->NDimIndexUIter<'a,DIM> {
        NDimIndexUIter::new(&self.lens)
    }
}

#[test]
fn test() {
    let a_compress_index=NDimIndexerU::new_len([10,10,10]);
    let a_compresss_1=a_compress_index.compress_index([1,2,3]);
    assert_eq!(a_compresss_1,321);//println!("{}",a_compresss_1);

    let a_seperated=a_compress_index.seperate_index(a_compresss_1);
    assert_eq!(a_seperated,[1,2,3]);//println!("{:?}",a_seperated);

    let b_compress_index=NDimIndexerU::new_len([1,2,3]);
    let b_compress_index_iter=b_compress_index.iter();
    assert_eq!(b_compress_index_iter.collect::<Vec<[usize;3]>>(),vec![
        [0, 0, 0],
        [0, 1, 0],
        [0, 0, 1],
        [0, 1, 1],
        [0, 0, 2],
        [0, 1, 2]
    ]);
    // for b_compress_index in b_compress_index_iter {
    //     println!("{:?}",b_compress_index)
    // }
    
}