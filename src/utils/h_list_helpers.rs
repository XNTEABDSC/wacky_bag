//! some helper things for h_list

use std::{iter::Chain, marker::PhantomData, ops::{Add, Deref, DerefMut, Neg}};

use frunk::{Func, HCons, HList, HNil, Poly, ToMut, ToRef, hlist::{self, HMappable, HZippable}};

use crate::{impl_phantom, new_struct_func, utils::type_fn::{BijectiveFunc, BijectiveTypeFunc, ChainFunc, MapFromPhantomPanic, TypeFnAsPhantomFn, TypeFunc}};

new_struct_func!(
	pub MapToPhantom
	impl<T>: 
	(T) <-> (PhantomData<T>) 
	|_i|Default::default()
);

// `(Acc,X)` -> `Chain<Acc,X>`
// pub struct FoldChainIter;

new_struct_func!(
	pub FoldChainIter impl <Acc,X,Item>
	{where Acc:Iterator<Item = Item>,
		X:Iterator<Item = Item>}
	:
	((Acc,X)) <-> (Chain<Acc,X>)
	|i|i.0.chain(i.1)
);

// impl_func!(
// 	<Acc,X,Item> for FoldChainIter {
// 	where Acc:Iterator<Item = Item>,
// 		X:Iterator<Item = Item>
// 	}:
// 	<(Acc,X)> <=> <Chain<Acc,X>>
// 	|i|i.0.chain(i.1)
// );

// impl<Acc,X,Item> Func<(Acc,X)> for FoldChainIter 
// 	where Acc:Iterator<Item = Item>,
// 		X:Iterator<Item = Item>
// {
// 	type Output=Chain<Acc,X>;

// 	fn call(i: (Acc,X)) -> Self::Output {
// 		i.0.chain(i.1)
// 	}
// }

new_struct_func!{
	pub MapDeref
	impl <'a,TA,TB>
	{where TA:Deref<Target=TB>+'a,TB:'a}:
	(&'a TA) -> (&'a TB)
	|i|i.deref()
}

// /// `x` -> `x.deref()`
// pub struct MapDeref;

// impl<'a,TA,TB> TypeFunc<&'a TA> for MapDeref
// 	where TA:Deref<Target=TB>,TB:'a
// {
// 	type Output=&'a TB;
// }

// impl<'a,TA,TB> Func<&'a TA> for MapDeref
// 	where TA:Deref<Target = TB>,TB:'a
// {
// 	type Output=&'a TB;

// 	fn call(i: &'a TA) -> Self::Output {
// 		i.deref()
// 	}
// }


// new_struct_func!{
// 	pub MapDerefT<TF>
// 	impl <'a,TF,TA,TB> {
// 	where 
// 		TA:Deref<Target=TB>,
// 		TB:'a,
// 		TF:TypeFunc<TA,Output = TB>
// 	}:
// 	<&'a TA> <=> <&'a TB>
// 	|i|i.deref()
// }

/// `ta:TA` -> `ta.deref():TB`, with TF: TA <-> TB specified by `TF`
pub struct MapDerefT<TF>(PhantomData<TF>);

impl<TF> std::fmt::Debug for MapDerefT<TF> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("MapDerefT").field(&self.0).finish()
	}
}

impl<TF> core::marker::Copy for MapDerefT<TF> {}

impl<TF> core::clone::Clone for MapDerefT<TF> {
    fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<TF> core::default::Default for MapDerefT<TF> {
    fn default() -> Self {
		Self(core::default::Default::default())
	}
}

impl<'a,TF,TA,TB> TypeFunc<&'a TA> for MapDerefT<TF> 
	where 
		TA:Deref<Target=TB>,
		TB:'a,
		TF:TypeFunc<TA,Output = TB>
{
	type Output=&'a TB;
}

impl<'a,TF,TA,TB> BijectiveTypeFunc<&'a TB> for MapDerefT<TF> 
	where 
		TA:Deref<Target=TB>,
		TB:'a,TA:'a,
		TF:BijectiveTypeFunc<TB,Input = TA>
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

new_struct_func!{
	pub MapClone
	impl <'a,T> {where T:Clone+'a}:
	(&'a T) -> (T)
	|i|i.clone()
}

// /// `&'a i` -> `i.clone()`
// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapClone;

// impl<'a,T> Func<&'a T> for MapClone
// 	where T:Clone+'a
// {
// 	type Output=T;

// 	fn call(i: &'a T) -> Self::Output {
// 		i.clone()
// 	}
// }

new_struct_func!{
	pub MapMutToRef
	impl<'a,T> {where T:'a}:
	(&'a mut T) <-> (&'a T)
	|i|i
}

// `&mut i` -> `&i`
// pub struct MapMutToRef;

// impl<'a,T> TypeFunc<&'a mut T> for MapMutToRef {
// 	type Output=&'a T;
// }

// impl<'a,T> BijectiveTypeFunc<&'a T> for MapMutToRef {
// 	type Input=&'a mut T;
// }

// impl<'a,T> Func<&'a mut T> for MapMutToRef {
// 	type Output=&'a T;

// 	fn call(i: &'a mut T) -> Self::Output {
// 		i
// 	}
// }

new_struct_func!{
	pub MapNeg
	impl <T> {where T:Neg}:
	(T) -> (T::Output)
	|i|-i
}

// /// `i` -> `-i`
// pub struct MapNeg;
// impl<T> TypeFunc<T> for MapNeg 
// 	where T:Neg
// {
// 	type Output=T::Output;
// }
// impl<T,O> Func<T> for MapNeg 
// 	where T:Neg<Output = O>
// {
// 	type Output=O;

// 	fn call(i: T) -> Self::Output {
// 		-i
// 	}
// }

new_struct_func!{
	pub MapNegRev
	impl <T1,T2>
	{where T1:Neg<Output = T2>,
		T2:Neg<Output = T1>}:
	(T1) <-> (T2)
	|i|-i,
	|i|-i
}

// /// `a` -> `-b`, `b` -> `-a`
// pub struct MapNegRev;

// impl<T1,T2> TypeFunc<T1> for MapNegRev
// 	where T1:Neg<Output = T2>,
// 		T2:Neg<Output = T1>
// {
// 	type Output=T2;
// }
// impl<T1,T2> BijectiveTypeFunc<T2> for MapNegRev
// 	where T1:Neg<Output = T2>,
// 		T2:Neg<Output = T1>
// {
// 	type Input=T1;
// }

// impl<T1,T2> Func<T1> for MapNegRev 
// 	where T1:Neg<Output = T2>,
// 		T2:Neg<Output = T1>
// {
// 	type Output=T2;

// 	fn call(i: T1) -> Self::Output {
// 		-i
// 	}
// }

// impl<T1,T2> BijectiveFunc<T2> for MapNegRev 
// 	where T1:Neg<Output = T2>,
// 		T2:Neg<Output = T1>
// {
// 	type Input=T1;

// 	fn inv_call(output:T2)->Self::Input {
// 		-output
// 	}
// }

new_struct_func!{
	pub MapRef<'a>
	impl<T> {where T:'a}:
	(T) <-> (&'a T)
}

// /// [TypeFunc] `T` <-> `&'a T`
// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapRef<'a>(pub PhantomData<&'a ()>);

// impl<'a,T:'a> TypeFunc<T> for MapRef<'a> {
// 	type Output=&'a T;
// }
// impl<'a,T:'a> BijectiveTypeFunc<&'a T> for MapRef<'a> {
// 	type Input=T;
// }

new_struct_func!{
	pub MapFromRef
	impl<'a,T>:
	(&'a T) -> (T)
}

// /// [TypeFunc] `&T` -> `T`
// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapFromRef;
// impl<'a,T> TypeFunc<&'a T> for MapFromRef {
// 	type Output=T;
// }

new_struct_func!{
	pub MapMut<'a>
	impl<T> {where T:'a}:
	(T) <-> (&'a mut T)
}

// /// [TypeFunc] `T` -> `&'a mut T`
// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapMut<'a>(pub PhantomData<&'a ()>);

// impl<'a,T:'a> TypeFunc<T> for MapMut<'a> {
// 	type Output=&'a mut T;
// }
// impl<'a,T:'a> BijectiveTypeFunc<&'a mut T> for MapMut<'a> {
// 	type Input=T;
// }

new_struct_func!{
	pub SetMut
	impl<'a,T>:
	((&'a mut T,T)) -> (())
	|i|{*i.0=i.1;}
}

// /// `|i: (&mut T,T)| {*i.0=i.1;}`
// #[derive(Debug,Default,Clone, Copy)]
// pub struct SetMut;

// impl<'a,T> Func<(&'a mut T,T)> for SetMut {
// 	type Output=();

// 	fn call(i: (&'a mut T,T)) -> Self::Output {
// 		*i.0=i.1;
// 	}
// }


new_struct_func!{
	pub FoldVecPush
	impl<VD,T> {where VD:Deref<Target = Vec<T>>+DerefMut}:
	((VD,T)) -> (VD)
	|mut i|{
		i.0.push(i.1);
		i.0
	}
}

// /// `(v,t):(&mut Vec<T>, T)` -> `v.push(t)`
// #[derive(Debug,Default,Clone, Copy)]
// pub struct FoldVecPush;

// impl<VD,T> Func<(VD,T)> for FoldVecPush
// 	where VD:Deref<Target = Vec<T>>+DerefMut
// {
// 	type Output=VD;

// 	fn call(mut i: (VD,T)) -> Self::Output {
// 		i.0.push(i.1);
// 		i.0
// 	}
// }

/// `<HList as HMappable<Mapper>>::Output`
/// 
/// [HMappable]
pub type HMap<HList,Mapper>=<HList as HMappable<Mapper>>::Output;
/// `<HList as HMappable<Poly<Mapper>>>::Output`
/// 
/// [HMappable]
pub type HMapP<HList,Mapper>=<HList as HMappable<Poly<Mapper>>>::Output;
/// `<A as HZippable<B>>::Zipped`
/// 
/// [HZippable]
pub type HZip<A,B>=<A as HZippable<B>>::Zipped;
/// `<T as ToRef<'a>>::Output`
/// 
/// [`ToRef`]
pub type HToRef<'a,T>=<T as ToRef<'a>>::Output;
/// `<T as ToMut<'a>>::Output`
/// 
/// [`ToMut`]
pub type HToMut<'a,T>=<T as ToMut<'a>>::Output;
/// `<A as Add<B>>::Output`
/// 
/// [Add]
pub type Sum<A,B>=<A as Add<B>>::Output;

/// map type `HList` by [TypeFunc] `TypeFn`
/// 
/// `HList` -> [MapToPhantom] -> `TypeFunc` -> [MapFromPhantomPanic]
/// 
/// `HMapP<HMapP<HMapP<HList,MapToPhantom>,TypeFn>,MapPhantomType>`
pub type HTypeMapP<HList,TypeFunc> = HMapP<HMapP<HMapP<HList,MapToPhantom>,TypeFunc>,MapFromPhantomPanic>;

/// Convert `TypeFn` from [TypeFunc] to [Func] that can be used in [HMapP]
/// 
/// PANICS WHEN USED IN [frunk::HCons::map] AS `mapper`
/// 
/// [MapToPhantom] * `TypeFn` * [MapFromPhantomPanic]
/// 
/// `ChainFunc<ChainFunc<MapToPhantom,TypeFn>,MapPhantomType>`
pub type HTypeFnToMapper<TypeFn> = ChainFunc<ChainFunc<MapToPhantom,TypeFnAsPhantomFn<TypeFn>>,MapFromPhantomPanic>;

/// repeat `Input` to get `Self`
/// 
/// see [`HRepeat`] [`h_repeat`] for usage
pub trait HRepeatFrom<Input> {
	/// repeat `Input` to get `Self`
	fn output_repeat(v:Input)->Self;
} 

impl<T> HRepeatFrom<T> for HNil {
	fn output_repeat(_v:T)->Self {
		HNil
	}
}

impl<V,T> HRepeatFrom<V> for HCons<V,T>
	where T:HRepeatFrom<V>,
	V:Clone
{
	fn output_repeat(v:V)->Self {
		HCons { head: v.clone(), tail: T::output_repeat(v) }
	}
}

/// a weird way to infer a type (H) that [`HRepeatFrom`]<T>
/// 
/// see [`HRepeat`] [`h_repeat`]

pub struct HInferRepeatFromStruct<H,T>(PhantomData<(H,T)>)
where H:HRepeatFrom<T>
;

impl_phantom!(HInferRepeatFromStruct<H,T> where H:HRepeatFrom<T>);

// impl<H, T> Default for HInferRepeatFromStruct<H, T>
// where H:HRepeatFrom<T>
// {
//     fn default() -> Self {
// 		Self(Default::default())
// 	}
// }

// impl<H, T> Clone for HInferRepeatFromStruct<H, T>
// where H:HRepeatFrom<T>
// {
//     fn clone(&self) -> Self {
// 		Self(self.0.clone())
// 	}
// }

// impl<H, T> Copy for HInferRepeatFromStruct<H, T>
// where H:HRepeatFrom<T>
// {
// }

/// a weird trait to infer a type (H) that [`HRepeatFrom`]<T>
/// 
/// see [`HRepeat`] [`h_repeat`]
pub trait HInferRepeatFromTrait {
	/// inferred type that [`HRepeatFrom`]<T>
	type Repeated;
}

impl<H,T> HInferRepeatFromTrait for HInferRepeatFromStruct<H,T>
where H:HRepeatFrom<T>
{
	type Repeated=H;
}

/// `HRepeat<_,T>` gets a type that filled with T, and length inferred fron usage
/**
# Example
```    
# use frunk::hlist;
# use wacky_bag::utils::default_of::default;
# use wacky_bag::utils::h_list_helpers::HRepeat;
assert_eq!(
	hlist![1,2.0,"3"].zip(
		default::<HRepeat<_,()>>()
	),
	hlist![(1,()),(2.0,()),("3",())]
)
```
 */
pub type HRepeat<H,V>=<HInferRepeatFromStruct<H, V> as HInferRepeatFromTrait>::Repeated;

/// repeat v
pub fn h_repeat<H,V>(v:V)->HRepeat<H,V>
	where H:HRepeatFrom<V>
{
	H::output_repeat(v)
}


// impl<T> Func<T> for MapToHList {
// 	type Output=HList!(T);

// 	fn call(i: T) -> Self::Output {
// 		HCons{head:i,tail:HNil}
// 	}
// }

// impl<T> BijectiveFunc<HList!(T)> for MapToHList {
// 	type Input=T;

// 	fn inv_call(output:HList!(T))->Self::Input {
// 		output.head
// 	}
// }

// impl_func!{<T> for MapToHList <T> <=> <HList!(T)>}

// pub struct MapToHList;
new_struct_func!(pub MapToHList impl <T> : (T) <-> (HList!(T)) |i|HCons{head:i,tail:HNil},|h|h.head );

new_struct_func!{pub MapClone2 < 'a > impl < T > {where T:'a+Clone} : (&'a T) <-> (T) |i|i.clone()}

// /*
/// `&'a T` <-> `T`
/// `&'a i` -> `i.clone()`
// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapClone2<'a>(pub PhantomData<&'a ()>);

// impl_phantom!(MapClone2<'a>);

// impl<'a,T> BijectiveTypeFunc<T> for MapClone2<'a> 
// 	where T:Clone+'a
// {
// 	type Input=&'a T;
// }

// impl<'a,T> TypeFunc<&'a T> for MapClone2<'a>{
// 	type Output=T;
// }

// impl<'a,T> Func<&'a T> for MapClone2<'a>
// 	where T:Clone+'a
// {
// 	type Output=T;

// 	fn call(i: &'a T) -> Self::Output {
// 		i.clone()
// 	}
// }
// impl_func!(<'a,T:'a> for MapClone2<'a> : <&'a T> <=> <T> );
// */

#[cfg(test)]
mod test{
    use frunk::hlist;

	use crate::utils::default_of::default;

	use super::*;

	#[test]
	fn test() {
		assert_eq!(
			hlist![1,2.0,"3"].zip(
				default::<HRepeat<_,()>>()
			),
			hlist![(1,()),(2.0,()),("3",())]
		)
	}
}