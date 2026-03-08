use std::{iter::Chain, marker::PhantomData, ops::Deref};

use frunk::Func;

use crate::utils::type_fn::{OneOneMappingFunc, OneOneMappingTypeFunc, TypeFunc};

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
#[derive(Debug,Default,Clone, Copy)]
pub struct MapClone<'a>(pub PhantomData<&'a ()>);


impl<'a,T> TypeFunc<&'a T> for MapClone<'a>
	where T:Clone+'a
{
	type Output=T;
}

impl<'a,T> OneOneMappingTypeFunc<T> for MapClone<'a> 
	where T:Clone+'a
{
	type Input=&'a T;
}

impl<'a,T> Func<&'a T> for MapClone<'a>
	where T:Clone+'a
{
	type Output=T;

	fn call(i: &'a T) -> Self::Output {
		i.clone()
	}
}

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