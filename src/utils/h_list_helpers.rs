use std::{iter::Chain, marker::PhantomData};

use frunk::Func;

use crate::utils::type_fn::{OneOneMappingTypeFunc, TypeFunc};

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