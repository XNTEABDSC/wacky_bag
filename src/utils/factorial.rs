use std::sync::{LazyLock, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use either::Either;
use simba::scalar::RealField;
use crate::utils::num_extend::NumExtends;

static FRACTIONAL_VEC:LazyLock<RwLock<Vec<usize>>>=LazyLock::new(||RwLock::new(Vec::new()));

pub fn factorial(v:usize)->
Result<usize,
Either<
	PoisonError<RwLockReadGuard<'static, Vec<usize>>>,
	PoisonError<RwLockWriteGuard<'static,Vec<usize>>>
>>{
	let vec_read=FRACTIONAL_VEC.read().map_err(|v|Either::Left(v))?;
	let res_may=vec_read.get(v);
	if let Some(res)=res_may {
		return Ok(*res);
	}
	drop(vec_read);
	let mut vec=FRACTIONAL_VEC.write().map_err(|v|Either::Right(v))?;

	if vec.len()==0 {
		vec.push(1);
	}

	let mut prev=*vec.last().unwrap();

	for i in vec.len()..=v {
		prev=prev*i;
		vec.push(prev);
	}

	return Ok(prev);

}


static GAMMA_ADD_FRAC_1_2_FACTOR_VEC:LazyLock<RwLock<Vec<(usize,i32)>>>=LazyLock::new(||RwLock::new(Vec::new()));

/// Γ(n+1/2) / \sqrt(\pi)
pub fn gamma_add_frac_1_2_factor(n:usize)->
Result<(usize,i32),
Either<
	PoisonError<RwLockReadGuard<'static, Vec<(usize,i32)>>>,
	PoisonError<RwLockWriteGuard<'static,Vec<(usize,i32)>>>
>>{
	let vec_read=GAMMA_ADD_FRAC_1_2_FACTOR_VEC.read().map_err(|v|Either::Left(v))?;
	if let Some(res)=vec_read.get(n) {
		return Ok(*res);
	}
	drop(vec_read);
	let mut vec=GAMMA_ADD_FRAC_1_2_FACTOR_VEC.write().map_err(|v|Either::Right(v))?;
	if vec.len()==0 {
		vec.push((1,0));
	}
	let mut cur=*vec.last().unwrap();
	let mut cur_time=vec.len()*2-1;
	for _i in vec.len()..=n {
		cur=(cur.0*cur_time,cur.1+1);
		// while cur.0&1==0 {
		// 	cur=(cur.0>>1,cur.1-1);
		// }
		cur_time+=2;
		vec.push(cur);
	}
	return Ok(cur);

}
/// Gamma( n_timed_2 / 2 )
pub fn gamma_n_timed_2<Num:RealField>(n_timed_2:usize)->Num{
	if n_timed_2%2==0 {
		let n=n_timed_2>>1;
		return Num::from_isize(factorial(n-1).unwrap() as isize).unwrap();
	}else{
		let n=n_timed_2>>1;
		let res=gamma_add_frac_1_2_factor(n).unwrap();
		return Num::from_isize(res.0 as isize).unwrap()*Num::frac_1_2().powi(res.1)* Num::sqrt(Num::pi());
	}
}

#[cfg(test)]
mod test{
	use super::*;
	#[test]
	fn test_fact(){
		for i in 3..6 {
			print!("{} ",factorial(i).unwrap());
		}
	}

	#[test]
	fn test_gamma(){
		for i in 1..8 {
			println!("Gamma({}): {}",i as f32/2.0, gamma_n_timed_2::<f32>(i))
		}
	}

}