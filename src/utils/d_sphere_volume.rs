//! [`d_sphere_volume_c`]

use std::{collections::HashMap, sync::{LazyLock, RwLock}};

use simba::scalar::RealField;

use crate::{collections::type_map::TypeMap, utils::{factorial::gamma_n_timed_2, rw_lock_error_either::RwLockErrorEither}};



static D_SPHERE_MEASURE:LazyLock<RwLock< TypeMap >>=LazyLock::new(||Default::default());


/// calculates measure/factor of N dim sphere
pub fn d_sphere_measure<Num:RealField+Clone>(dim:usize)->Result<
Num,
RwLockErrorEither<'static,TypeMap>
>{
	// let type_id=TypeId::of::<HashMap<usize,Num>>();
	// let type_id=TypeId::of::<Num>();
	let d_sphere_measure_read=D_SPHERE_MEASURE.read()?;
	if let Some(a)=d_sphere_measure_read.get::<HashMap<usize,Num>>(){
		if let Some(v)=a.get(&dim) {
			return Ok(v.clone());
		}
	}
	// if let Some(a)=d_sphere_measure_read.get( &type_id ) {
	// 	let a=a.downcast_ref::<HashMap<usize,Num>>().unwrap();
	// 	if let Some(v)=a.get(&dim) {
	// 		return Ok(v.clone());
	// 	}
	// }
	drop(d_sphere_measure_read);
	let mut d_sphere_measure = D_SPHERE_MEASURE.write()?;
	let num_hm=d_sphere_measure.entry::<HashMap<usize,Num>>().or_insert_with(||HashMap::<usize,Num>::new());
	// let num_hm=num_hm_dyn.downcast_mut::<HashMap<usize,Num>>().unwrap();
	let res=num_hm.entry(dim).or_insert_with(||
		Num::pi().sqrt().powi(dim as i32) / gamma_n_timed_2(dim+2)
	);
	return Ok(res.clone());
	// d_sphere_measure.get_mut( type_id )
	// 	.map_or_else(
	// 		||{
	// 			let new_hm:HashMap<usize,Num>=Default::default();

	// 		}, 
	// 		|v|v.downcast_mut::<HashMap<usize,Num>>().unwrap());

	// return todo!();
	// let d_sphere_measure
}

/// calculates volume of N dim sphere by `radius.pow(dim)`
pub fn d_sphere_volume_by_radius_pow<Num:RealField>(radius_pow:Num,dim:usize)->Num{
	d_sphere_measure::<Num>(dim).unwrap()*radius_pow
}

/// calculates volume of N dim sphere
pub fn d_sphere_volume_c<Num:RealField+Clone,const DIM:usize>(radius:Num)->Num{
	d_sphere_measure::<Num>(DIM).unwrap()*radius.powi(DIM as i32)
}

#[cfg(test)]
mod test{

	use core::f32;

use approx::assert_relative_eq;
use simba::scalar::RealField;

	use crate::utils::d_sphere_volume::d_sphere_measure;

	pub fn test_for_num<Num:RealField+Copy>(){
		let pi=Num::pi();
		assert_relative_eq!(d_sphere_measure::<Num>(2).unwrap(), pi);
		assert_relative_eq!(d_sphere_measure::<Num>(3).unwrap(), pi*Num::from_isize(4).unwrap()/Num::from_isize(3).unwrap());
		assert_relative_eq!(d_sphere_measure::<Num>(4).unwrap(), pi*pi/Num::from_isize(2).unwrap());
	}

	#[test]
	fn test(){
		test_for_num::<f32>();
		test_for_num::<f64>();
	}
}