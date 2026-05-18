//! [TypeMap]

use std::{any::{Any, TypeId}, collections::{HashMap, hash_map}, hash::{BuildHasher, RandomState}, marker::PhantomData, ops::{ControlFlow, Deref}};

type AnyHashMap<S>=HashMap<TypeId,Box<dyn Any+Send+Sync>,S>;

/// `TypeMap` is a [`HashMap`] storing [Box] [Any], and get via type.
#[derive(Debug)]
pub struct TypeMap<S = RandomState>(AnyHashMap<S>);

impl Default for TypeMap<RandomState> {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl TypeMap<RandomState> {
	/// Creates an empty `TypeMap`.
	#[inline]
	pub fn new()->Self{Self::default()}
}

impl<S> TypeMap<S> {
	/// Creates an `TypeMap` via a [`HashMap`] without checking. 
	/// 
	/// For a safe alternative see [`from_hash_map`].
	/// 
	/// You can check via [`TypeMap::check_any_hash_map`]
	/// 
	/// Methods accessing `TypeMap` will return `None` if items' type not matching their index.
	#[inline]
	pub unsafe fn from_hash_map_unchecked(hash_map:AnyHashMap<S>)->Self{
		Self(hash_map)
	}
	/// Creates an `TypeMap` via a [HashMap] with checking.
	#[inline]
	pub fn from_hash_map(hash_map:AnyHashMap<S>)->Option<Self>{
		if Self::check_any_hash_map(&hash_map) {
			Some(Self(hash_map))
		}else {
			None
		}
		
	}
	/// Check whether all [HashMap]'s values' [TypeId] matches its key.
	/// 
	/// for all `(k,v)` in `&hash_map`, `(v.deref()).type_id()==*k` 
	#[inline]
	pub fn check_any_hash_map(hash_map:&AnyHashMap<S>)->bool{
		hash_map.iter().try_for_each(|(k,v)|{if (v.deref()).type_id()==*k {ControlFlow::Continue(())} else {ControlFlow::Break(())}}).is_continue()
	}
}
/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`TypeMap`].
///
/// [`entry`]: TypeMap::entry
pub struct Entry<'a,V:'a>{
	entry:hash_map::Entry<'a,TypeId,Box<dyn Any+Send+Sync>>,
	_p:PhantomData<V>
}

impl<S:BuildHasher> TypeMap<S> {
	/// Get ref of the inner [`HashMap`]
	#[inline]
	pub const fn inner(&self)->&AnyHashMap<S>{&self.0}
	/// Get mut ref of the inner [`HashMap`]
	/// 
	/// # Safety
	/// 
	/// Mutating the [`HashMap`] without guaranteeing [TypeMap::check_any_hash_map] will not causing crashes, but [`TypeMap`] just ignores the items.
	#[inline]
	pub const unsafe fn inner_mut(&mut self)->&mut AnyHashMap<S>{&mut self.0}
	/// Into the inner [`HashMap`]
	#[inline]
	pub fn into_inner(self)->AnyHashMap<S> {
		self.0
	}
	/// Gets the given types's corresponding entry in the map for in-place manipulation.
	/// 
	/// Its similar to [HashMap::entry]
	#[inline]
	pub fn entry<'a,T:'static>(&'a mut self)->Entry<'a,T> {
		Entry { entry: self.0.entry(TypeId::of::<T>()), _p: Default::default() }
	}
	/// Gets the ref value via type. 
	#[inline]
	pub fn get<T:'static>(&self)->Option<&T>{
		self.0.get(&TypeId::of::<T>()).and_then(|v|v.downcast_ref::<T>())
	}
	/// Gets the mut ref value via type. 
	#[inline]
	pub fn get_mut<T:'static>(&mut self)->Option<&mut T>{
		self.0.get_mut(&TypeId::of::<T>()).and_then(|v|v.downcast_mut::<T>())
	}
	/// Insert a value, returning the value of same type if exist.
	#[inline]
	pub fn insert<T:'static+Send+Sync>(&mut self,v:T)->Option<Box<T>>  {
		self.0.insert(TypeId::of::<T>(), Box::new(v)).and_then(|v|v.downcast().ok())
	}
	/// Remove a value of the type, returning the value of same type if exist.
	#[inline]
	pub fn remove<T:'static>(&mut self)->Option<Box<T>>{
		self.0.remove(&TypeId::of::<T>()).and_then(|v|v.downcast().ok())
	}

}

impl<'a,V:'static+Send+Sync> Entry<'a,V> {
	/// if the value dont exist, call `f` to get the value.
	/// 
	/// then return `&'a mut V` inside [`TypeMap`]
	#[inline]
	pub fn or_insert_with<F:FnOnce()->V>(self,f:F)->&'a mut V{
		self.entry.or_insert_with(|| Box::new(f())).downcast_mut().unwrap()
	}
	/// Get the [`TypeId`] (key)
	#[inline]
	pub fn key(&self)->&TypeId{
		self.entry.key()
	}
	/// If the value already exist, modify it via `F`
	#[inline]
	pub fn and_modify<F:FnOnce(&mut V)>(self,f:F)->Self{
		Self { entry: self.entry.and_modify(|v|{
			f(v.downcast_mut().unwrap())
		}), _p: Default::default() }
	}
	
	/// if the value dont exist, insert a [`Default`] value.
	/// 
	/// then return `&'a mut V` inside [`TypeMap`]
	#[inline]
	pub fn or_default(self)->&'a mut V
		where V:Default
	{
		self.or_insert_with(||V::default())
	}
	/// if the value dont exist, insert `v`.
	/// 
	/// then return `&'a mut V` inside [`TypeMap`]
	#[inline]
	pub fn or_insert(self,v:V)->&'a mut V{
		self.or_insert_with(move ||v)
	}
}