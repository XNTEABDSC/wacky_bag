use std::{iter::Chain, marker::PhantomData, ops::{Deref, Neg}};

use frunk::{Func, Poly, ToMut, ToRef, hlist::{HMappable, HZippable}};

use crate::utils::type_fn::{OneOneMappingFunc, OneOneMappingTypeFunc, TypeFunc};

/// `T` <-> `Phantom<T>`
pub struct MapToPhantom;

impl<T> TypeFunc<T> for MapToPhantom{
	type Output=PhantomData<T>;
}

impl<T> OneOneMappingTypeFunc<PhantomData<T>> for MapToPhantom {
	type Input=T;
}

impl<T> Func<T> for MapToPhantom {
	type Output=PhantomData<T>;

	fn call(_i: T) -> Self::Output {
		PhantomData::default()
	}
}

/// `(Acc,X)` -> `Chain<Acc,X>`
pub struct FoldChainIter;

impl<Acc,X,Item> Func<(Acc,X)> for FoldChainIter 
	where Acc:Iterator<Item = Item>,
		X:Iterator<Item = Item>
{
	type Output=Chain<Acc,X>;

	fn call(i: (Acc,X)) -> Self::Output {
		i.0.chain(i.1)
	}
}

// `x` -> `x.deref()`
pub struct MapDeref;

impl<'a,TA,TB> TypeFunc<&'a TA> for MapDeref
	where TA:Deref<Target=TB>,TB:'a
{
	type Output=&'a TB;
}

impl<'a,TA,TB> Func<&'a TA> for MapDeref
	where TA:Deref<Target = TB>,TB:'a
{
	type Output=&'a TB;

	fn call(i: &'a TA) -> Self::Output {
		i.deref()
	}
}


// `ta:TA` -> `ta.deref():TB`, with TF: TA <-> TB specifying
pub struct MapDerefT<TF>(PhantomData<TF>);

impl<'a,TF,TA,TB> TypeFunc<&'a TA> for MapDerefT<TF> 
	where 
		TA:Deref<Target=TB>,
		TB:'a,
		TF:TypeFunc<TA,Output = TB>
{
	type Output=&'a TB;
}

impl<'a,TF,TA,TB> OneOneMappingTypeFunc<&'a TB> for MapDerefT<TF> 
	where 
		TA:Deref<Target=TB>,
		TB:'a,TA:'a,
		TF:OneOneMappingTypeFunc<TB,Input = TA>
{
	type Input =&'a TA;
}

impl<'a,TF,TA,TB> Func<&'a TA> for MapDerefT<TF>
	where 
		TA:Deref<Target=TB>,
		TB:'a,
		TF:TypeFunc<TA,Output = TB>

{
	type Output=&'a TB;

	fn call(i: &'a TA) -> Self::Output {
		i.deref()
	}
}

/// `&'a i` -> `i.clone()`
#[derive(Debug,Default,Clone, Copy)]
pub struct MapClone;


impl<'a,T> TypeFunc<&'a T> for MapClone
	where T:Clone+'a
{
	type Output=T;
}

impl<'a,T> Func<&'a T> for MapClone
	where T:Clone+'a
{
	type Output=T;

	fn call(i: &'a T) -> Self::Output {
		i.clone()
	}
}

/// `&'a T` <-> `T`
/// `&'a i` -> `i.clone()`
#[derive(Debug,Default,Clone, Copy)]
pub struct MapClone2<'a>(pub PhantomData<&'a ()>);

impl<'a,T> TypeFunc<&'a T> for MapClone2<'a>
	where T:Clone+'a
{
	type Output=T;
}

impl<'a,T> OneOneMappingTypeFunc<T> for MapClone2<'a> 
	where T:Clone+'a
{
	type Input=&'a T;
}

impl<'a,T> Func<&'a T> for MapClone2<'a>
	where T:Clone+'a
{
	type Output=T;

	fn call(i: &'a T) -> Self::Output {
		i.clone()
	}
}
/// `&mut i` -> `&i`
pub struct MapMutToRef;

impl<'a,T> TypeFunc<&'a mut T> for MapMutToRef {
	type Output=&'a T;
}

impl<'a,T> OneOneMappingTypeFunc<&'a T> for MapMutToRef {
	type Input=&'a mut T;
}

impl<'a,T> Func<&'a mut T> for MapMutToRef {
	type Output=&'a T;

	fn call(i: &'a mut T) -> Self::Output {
		i
	}
}
/// `i` -> `-i`
pub struct MapNeg;
impl<T> TypeFunc<T> for MapNeg 
	where T:Neg
{
	type Output=T::Output;
}
impl<T,O> Func<T> for MapNeg 
	where T:Neg<Output = O>
{
	type Output=O;

	fn call(i: T) -> Self::Output {
		-i
	}
}

/// `a` -> `-b`, `b` -> `-a`
pub struct MapNegRev;

impl<T1,T2> TypeFunc<T1> for MapNegRev
	where T1:Neg<Output = T2>,
		T2:Neg<Output = T1>
{
	type Output=T2;
}
impl<T1,T2> OneOneMappingTypeFunc<T2> for MapNegRev
	where T1:Neg<Output = T2>,
		T2:Neg<Output = T1>
{
	type Input=T1;
}

impl<T1,T2> Func<T1> for MapNegRev 
	where T1:Neg<Output = T2>,
		T2:Neg<Output = T1>
{
	type Output=T2;

	fn call(i: T1) -> Self::Output {
		-i
	}
}

impl<T1,T2> OneOneMappingFunc<T2> for MapNegRev 
	where T1:Neg<Output = T2>,
		T2:Neg<Output = T1>
{
	type Input=T1;

	fn inv_call(output:T2)->Self::Input {
		-output
	}
}

/// `T` -> `&'a T`
#[derive(Debug,Default,Clone, Copy)]
pub struct MapRef<'a>(pub PhantomData<&'a ()>);

impl<'a,T:'a> TypeFunc<T> for MapRef<'a> {
	type Output=&'a T;
}
impl<'a,T:'a> OneOneMappingTypeFunc<&'a T> for MapRef<'a> {
	type Input=T;
}

/// `T` -> `&'a mut T`
#[derive(Debug,Default,Clone, Copy)]
pub struct MapMut<'a>(pub PhantomData<&'a ()>);

impl<'a,T:'a> TypeFunc<T> for MapMut<'a> {
	type Output=&'a mut T;
}
impl<'a,T:'a> OneOneMappingTypeFunc<&'a mut T> for MapMut<'a> {
	type Input=T;
}

pub type HMap<HList,Mapper>=<HList as HMappable<Mapper>>::Output;
pub type HMapP<HList,Mapper>=<HList as HMappable<Poly<Mapper>>>::Output;
pub type HZip<A,B>=<A as HZippable<B>>::Zipped;
pub type HToRef<'a,T>=<T as ToRef<'a>>::Output;
pub type HToMut<'a,T>=<T as ToMut<'a>>::Output;