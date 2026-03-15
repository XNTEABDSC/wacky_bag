use std::array;

use crate::utils::default_of::default;


#[derive(Debug, Clone, Copy, Default)]
pub struct DimDir{
	pub dim:usize,
	pub dir_positive:bool
}

impl DimDir {
	pub fn to_dir_vec<const DIM:usize>(&self)->[isize;DIM]{
		array::from_fn(|i|{
			if i==self.dim{
				if self.dir_positive{1}else{-1}
			}else {
				0
			}
		})
	}
}

pub struct DimDirIter{
	pub dim_dir:DimDir,
	max_dim:usize
}

impl DimDirIter {
	pub fn new(max_dim:usize)->DimDirIter{Self { dim_dir:default(), max_dim  }}
}

impl Iterator for DimDirIter {
	type Item=DimDir;

	fn next(&mut self) -> Option<Self::Item> {
		if self.dim_dir.dim>=self.max_dim {
			return None;
		}
		let res=self.dim_dir.clone();
		if self.dim_dir.dir_positive==false {
			self.dim_dir.dir_positive=true;
		}else {
			self.dim_dir.dim+=1;
			self.dim_dir.dir_positive=false;
		}
		return Some(res);
	}
}