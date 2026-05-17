// use crate::num::Num;

use std::sync::{LazyLock, RwLock};

use num_traits::FromPrimitive;
use simba::scalar::RealField;

use crate::collections::type_map::TypeMap;

pub const trait NormalCdfConsts<Marker>:Sized
{
	// const DATAS:NormalCdfConstsData<Self>;
	fn datas()->NormalCdfConstsData<Self>;
}

pub const NORMAL_CDF_CONSTS_DATA_F64:NormalCdfConstsData<f64>=
NormalCdfConstsData{
	a1:0.254829592,
	a2:-0.284496736,
	a3:1.421413741,
	a4:-1.453152027,
	a5:1.061405429,
	p:0.3275911,
};

pub struct NormalCdfConstsByFromF64;

pub static NORMAL_CDF_CONSTS_BY_FROM_F64:LazyLock<RwLock<TypeMap>>=LazyLock::new(||Default::default());

impl<T> NormalCdfConsts<NormalCdfConstsByFromF64> for T
	where T:FromPrimitive+Copy+Send+Sync+'static
{
	fn datas()->NormalCdfConstsData<Self> {
		let cs_r=NORMAL_CDF_CONSTS_BY_FROM_F64.read().unwrap();
		if let Some(cs)=cs_r.get::<NormalCdfConstsData<T>>() {
			return *cs;
		}else {
			drop(cs_r);
			let mut cs_w=NORMAL_CDF_CONSTS_BY_FROM_F64.write().unwrap();
			let res=cs_w.entry::<NormalCdfConstsData<T>>().or_insert_with(||{
				NORMAL_CDF_CONSTS_DATA_F64.map(|v|T::from_f64(v).unwrap())
			});
			*res
		}
	}
}
#[derive(Debug,Clone, Copy)]
pub struct NormalCdfConstsData<Num>
{
	pub a1:Num,
	pub a2:Num,
	pub a3:Num,
	pub a4:Num,
	pub a5:Num,
	pub p:Num
}

impl<Num> NormalCdfConstsData<Num> {
	pub const fn map_c<F,B>(&self,f:&F)->NormalCdfConstsData<B>
		where F: for<'a> const Fn(&'a Num)->B
	{
		NormalCdfConstsData{
			a1:f(&self.a1),
			a2:f(&self.a2),
			a3:f(&self.a3),
			a4:f(&self.a4),
			a5:f(&self.a5),
			p:f(&self.p),
		}
	}
	pub fn map<F,B>(self,f:F)->NormalCdfConstsData<B>
		where F:Fn(Num)->B
	{
		NormalCdfConstsData{
			a1:f(self.a1),
			a2:f(self.a2),
			a3:f(self.a3),
			a4:f(self.a4),
			a5:f(self.a5),
			p:f(self.p),
		}
	}
}

pub fn normal_cdf<Num,Marker>(x:Num)->Num 
	where Num:RealField+Copy+NormalCdfConsts<Marker>
{
	
	let v=<Num as NormalCdfConsts<Marker>>::datas();
	let NormalCdfConstsData{a1,a2,a3,a4,a5,p}=v;
	let two=Num::one()+Num::one();
    let sign = x.signum();
    // let x2=x.abs()*Num::FRAC_1_SQRT_2;
	let x2=x.abs()/Num::sqrt(two);

    let t= Num::one()/(Num::one()+p*x2);
    let y=Num::one()-(((((a5*t + a4)*t) + a3)*t + a2)*t + a1)*t*Num::exp(-x2*x2);
    return (Num::one()+sign*y)/two;
}

pub fn test_normal_cdf<Num,Marker>()->[Num;5]
	where Num:RealField+Copy+NormalCdfConsts<Marker>
{
	let xs=[
		(-3.0, 0.00134989803163),	
		(-1.0, 0.158655253931),
		( 0.0, 0.5),
		( 0.5, 0.691462461274),
		( 2.1 , 0.982135579437),
	];

	let xs_num=xs.map(|(a,b)|(Num::from_f64(a).unwrap(),Num::from_f64(b).unwrap()));
	let v=xs_num.map(|(a,b)|(normal_cdf(a)-b).abs());
	// return v.max_by(|a,b|a.partial_cmp(b).unwrap()).unwrap();
	return v;
}

#[test]
fn test_normal_cdf_f64(){
	println!("{:?}",test_normal_cdf::<f64,_>());
}