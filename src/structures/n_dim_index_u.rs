use std::{array, ops::{ControlFlow, Deref}};

use crate::structures::{just::Just, n_dim_index::TNDimIndexer};



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
        let iterate=cur.iter_mut().zip((&lens).into_iter()).rev().try_for_each(|(c,l)|{
            *c+=1;
            if *c>=*l{
                *c=0;
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

impl<const DIM:usize> TNDimIndexer<DIM> for NDimIndexerU<DIM> {
	fn length(&self) -> impl Deref<Target = usize> {
		&self.length
	}

	fn lens(&self)->impl Deref<Target = [std::ops::Range<isize>; DIM]> {
		Just(self.lens.map(|l|0isize..(l as isize)))
	}
	
	fn steps(&self)->&[usize;DIM] {
		&self.steps
	}

	fn contains(&self,indexes:&super::n_dim_index::NDimIndex<DIM>)->bool {
		self.lens.iter().zip(indexes.iter()).try_for_each(|(l,a)|{
			if (*a) < (*l as isize) {
				return ControlFlow::Continue(());
			}else {
				return ControlFlow::Break(());
			}
		}).is_continue()
	}

	fn contains_compressed(&self,index:usize)->bool {
		index<self.length
	}

	fn compress_index(&self,indexes:&super::n_dim_index::NDimIndex<DIM>)->usize {
		self.compress_index_u(indexes.map(|a|a as usize))
	}

	fn decompress_index(&self,compressed_index:usize)->super::n_dim_index::NDimIndex<DIM> {
		self.decompress_index_u(compressed_index).map(|a|a as isize)
	}

	fn iter<'a>(&'a self)->impl Iterator<Item=super::n_dim_index::NDimIndex<DIM>> + 'a {
		self.iter_u().map(|a|a.map(|b|b as isize))
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
		return (compressed_index/step) as isize;
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
			new_compressed+=self.length as isize;
		}
		while new_compressed>=self.length as isize {
			new_compressed-=self.length as isize;
		}
		return new_compressed as usize;
	}
}

impl<const DIM:usize> NDimIndexerU<DIM>{
    pub fn lens(&self)->&[usize;DIM]{&self.lens}
    pub fn bases(&self)->&[usize;DIM]{&self.steps}
    pub fn length(&self)->usize{self.length}

    pub fn new_len(lens:[usize;DIM])->Self{
        let mut steps=[0usize;DIM];
        let mut cur_length:usize=1;
        for i in (0..DIM).rev() {
            steps[i]=cur_length;
            let dim_len= lens[i];
            cur_length*=dim_len;
        }
        Self{steps,length:cur_length,lens}
    }

	pub fn contains_u(&self,indexes:NDimIndexU<DIM>)->bool{
		self.lens.iter().zip(indexes.iter()).try_for_each(|(l,a)|{
			if a<l {
				return ControlFlow::Continue(());
			}else {
				return ControlFlow::Break(());
			}
		}).is_continue()
	}

    pub fn compress_index_u(&self,indexes:NDimIndexU<DIM>)->usize{
        let mut res=0;
        for i in 0..DIM {
            res+=indexes[i]*self.steps[i]
        }
        res
    }
    /* */
    pub fn decompress_index_u(&self,mut compressed_index:usize)->NDimIndexU<DIM>{
        array::from_fn(|i|{
            let step=self.lens[i];
            let (div,rem)=(compressed_index/step,compressed_index%step);
            compressed_index=div;
            rem
        })
    }
    pub fn iter_u<'a>(&'a self)->NDimIndexUIter<'a,DIM> {
        NDimIndexUIter::new(&self.lens)
    }
}

#[test]
fn test() {
    let a_compress_index=NDimIndexerU::new_len([10,10,10]);
    let a_compresss_1=a_compress_index.compress_index_u([1,2,3]);
    assert_eq!(a_compresss_1,321);//println!("{}",a_compresss_1);

    let a_seperated=a_compress_index.decompress_index_u(a_compresss_1);
    assert_eq!(a_seperated,[1,2,3]);//println!("{:?}",a_seperated);

    let b_compress_index=NDimIndexerU::new_len([1,2,3]);
    let b_compress_index_iter=b_compress_index.iter_u();
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