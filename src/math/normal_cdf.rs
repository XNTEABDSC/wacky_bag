// use crate::num::Num;

use core::str::FromStr;
use std::ops::{Add, Deref, Div, Mul};

use simba::scalar::RealField;

pub const trait NormalCdfConsts<Marker>:Sized
{
	// const DATAS:NormalCdfConstsData<Self>;
	fn datas()->NormalCdfConstsData<Self>;
}

const trait NormalCdfConstsByFromStr:Sized+FromStr
{
	const DATAS:NormalCdfConstsDataMayErr<Self,Self::Err>;
}

pub const NORMAL_CDF_CONSTS_DATA_STR:NormalCdfConstsData<&'static str>=NormalCdfConstsData{
	a1:&"0.254829592",
	a2:&"-0.284496736",
	a3:&"1.421413741",
	a4:&"-1.453152027",
	a5:&"1.061405429",
	p:&"0.3275911",
};


pub const NORMAL_CDF_CONSTS_DATA_F64:NormalCdfConstsData<f64>=
NormalCdfConstsData{
	a1:0.254829592,
	a2:-0.284496736,
	a3:1.421413741,
	a4:-1.453152027,
	a5:1.061405429,
	p:0.3275911,
};

const fn map_from_str<Num:const FromStr>(v:&&str)->Result<Num,Num::Err>{
	FromStr::from_str(v)
}

impl<T> NormalCdfConstsByFromStr for T 
	where T:const FromStr
{
	const DATAS:NormalCdfConstsData<Result<Self,T::Err>>=
		NORMAL_CDF_CONSTS_DATA_STR.map_c(&map_from_str);
}

pub struct NormalCdfConstsByFromStrMarker;

impl<T> NormalCdfConsts<NormalCdfConstsByFromStrMarker> for T
	where T:NormalCdfConstsByFromStr+'static
{
	fn datas()->NormalCdfConstsData<Self> {
		T::DATAS.unwrap()
	}
}

pub type NormalCdfConstsDataMayErr<Num,Err>=NormalCdfConstsData<Result<Num,Err>>;

impl<Num,Err> NormalCdfConstsDataMayErr<Num,Err> {
	pub fn unwrap(self)->NormalCdfConstsData<Num>{
		let NormalCdfConstsDataMayErr{ a1:Ok(a1),a2:Ok(a2),a3:Ok(a3),a4:Ok(a4),a5:Ok(a5),p:Ok(p)}=self else {
			panic!("Unable to parse");
		};
		NormalCdfConstsData{a1,a2,a3,a4,a5,p}
	}
}

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
    // const A1: Result<Num, <Num as FromStr>::Err> = <Num as FromStr>::from_str("0.254829592");
    // const A2: Result<Num, <Num as FromStr>::Err> = Num::from_str("-0.284496736");
    // const A3: Result<Num, fixed::ParseFixedError> = Num::from_str("1.421413741");
    // const A4: Result<Num, fixed::ParseFixedError> = Num::from_str("-1.453152027");
    // const A5: Result<Num, fixed::ParseFixedError> = Num::from_str("1.061405429");
    // const P: Result<Num, fixed::ParseFixedError> = Num::from_str("0.3275911");
	
	let v=<Num as NormalCdfConsts<Marker>>::datas();
	let NormalCdfConstsData{a1,a2,a3,a4,a5,p}=v;
	let two=Num::one()+Num::one();
    let sign = x.signum();
    // let x2=x.abs()*Num::FRAC_1_SQRT_2;
	let x2=x.abs()/Num::sqrt(two);

    let t= Num::one()/(Num::one()+p*x2);
    let y=Num::one()-(((((a5*t + a4)*t) + a3)*t + a2)*t + a1)*t*Num::exp(-x*x);
    return (Num::one()+sign*y)/Into::<Num>::into(two);
}