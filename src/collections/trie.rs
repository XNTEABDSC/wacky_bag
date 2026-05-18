//! [Trie]
use std::{collections::HashMap, hash::Hash, iter::Chain, mem};

/// `Trie` allows walking through a tree via a [Iterator<Item = Key>].
/// 
/// `Trie` itself is also the node of the tree. It may contains a `TValue`
#[derive(Debug,Default)]
pub struct Trie<Key,Value>
    where Key:Hash+Eq
{
    value:Option<Value>,
    nexts:HashMap<Key,Trie<Key,Value>>
}

unsafe impl<Key: Sync, Value: Sync> Sync for Trie<Key, Value>
where Key:Hash+Eq
{
}

impl<Key,Value> Trie<Key,Value> 
    where Key:Hash+Eq
{
	/// Constructs a enpty `Trie`
    pub fn new()->Self {
        Self { nexts: Default::default(),value:None }
    }

	/// Travels through the `Trie` via a iterator until run out of the iterator, Returning `&Trie`
	/// 
	/// Not guarantees that the result contains `Value`.
    pub fn travel(&self,mut index:impl Iterator<Item = Key>)->Option<&Trie<Key,Value>> {
        let key_=index.next();
        if let Some(key)=key_{
            self.nexts.get(&key).map_or(None,|next|next.travel(index))
        }else {
            Some(self)
        }
    }

	/// Travels through the `Trie` via a iterator until run out of the iterator, Returning `&mut Trie`
	/// 
	/// Not guarantees that the result contains `Value`.
    pub fn travel_mut(&mut self,mut index:impl Iterator<Item = Key>)->Option<&mut Trie<Key,Value>> {
        let key_=index.next();
        if let Some(key)=key_{
            self.nexts.get_mut(&key).map_or(None,|next|next.travel_mut(index))
        }else {
            Some(self)
        }
    }
    /*
    fn travel_make(&mut self,mut index:impl Iterator<Item = Key>)->&mut Trie<Key,Value> {
        let key_=index.next();
        if let Some(key)=key_{
            let next_=self.nexts.get_mut(&key);
            if let Some(next)=next_{
                next.travel_make(index)
            }else {
                todo!();
                /*
                let mut next=Self::new();
                self.nexts.insert(key, next);
                let mut next=self.nexts.get_mut(&key).unwrap();
                let res=next.travel_make(index);
                res 
                */
            }
        }else {
            self
        }
    } */

	/// Insert a `value:Value` at `index: impl IntoIterator<Item = Key>`, returning `Value` if one exist at `index`
    pub fn insert(&mut self,index:impl IntoIterator<Item = Key>,value:Value)->Option<Value>{
		let mut index=index.into_iter();
        let key_=index.next();
        if let Some(key)=key_{
            let next_=self.nexts.get_mut(&key);
            if let Some(next)=next_{
                next.insert(index, value)
            }else {
                let mut next=Self::new();
                let res=next.insert(index, value);
                
                self.nexts.insert(key, next);
                res
            }
        }else {
            mem::replace(&mut self.value, Some(value))
        }
    }

	/// remove a `Value` at `index: impl IntoIterator<Item = Key>` and return it if exist.
    pub fn remove(&mut self,mut index:impl Iterator<Item = Key>)->Option<Value> {
        let key_=index.next();
        if let Some(key)=key_{
            let next_=self.nexts.get_mut(&key);
            if let Some(next)=next_{
                let res = next.remove(index);
                if next.is_empty(){
                    self.nexts.remove(&key);
                }
                res
            }else {
                None
            }
        }else {
            mem::replace(&mut self.value, None)
        }
    }

	/// get a `&Value` at `index: impl IntoIterator<Item = Key>` if exist.
    pub fn get(&self,index:impl Iterator<Item = Key>)->Option<&Value> {
        self.travel(index).map_or(None,|n|n.value.as_ref())
    }

	/// get a `&mut Value` at `index: impl IntoIterator<Item = Key>` if exist.
    pub fn get_mut(&mut self,index:impl Iterator<Item = Key>)->Option<&mut Value> {
        self.travel_mut(index).map_or(None,|n|n.value.as_mut())
    }

	/// Whether this `Trie` dont have value and dont have children.
	/// 
	/// Not checking whether its children is empty. so this is fast.
    pub fn is_empty(&self)->bool{
        return self.value.is_none()&&self.nexts.len()==0;
    }
	
	/// Whether both this `Trie` and its children dont contains value.
	/// 
	/// May be costy.
    pub fn is_true_empty(&self)->bool{
        // return self.value.is_none()&&self.nexts.len()==0;
		if self.value.is_some() {
			return false;
		}
		for c in &self.nexts {
			if !c.1.is_true_empty() {
				return false;
			}
		}
		return true;
    }

	/// Trying to find the nearest `Value` via `index: impl IntoIterator<Item = Key>`, 
	/// 
	/// returning (
	/// 
	/// the value if found (only can be `None` if self dont contains value),
	/// 
	/// `Vec<Key>` that index the value.
	/// 
	/// a iterator about remainings of `index`
	/// 
	/// )
    pub fn match_get<Iter:IntoIterator<Item = Key>>(&self,index:Iter)->(Option<&Value>,Vec<Key>,Chain< <Vec<Key> as IntoIterator >::IntoIter , Iter::IntoIter>) {
        let mut index=index.into_iter();
		let mut unused=Vec::<Key>::new();
		let mut used=Vec::<Key>::new();
        let mut current: &Trie<Key, Value>=self;
        let mut selected:&Trie<Key, Value>=self;

        loop {
            let next_key_=index.next();
            if let Some(next_key)=next_key_{
                let next_=current.nexts.get(&next_key);
                unused.push(next_key);
                if let Some(next)=next_{
                    if next.value.is_some(){
                        selected=next;
						used.append(&mut unused);
                    }else {
                        
                    }
                    current=next;
                }else {
                    break;
                }
            }else {
                break;
            }
        }
        return (selected.value.as_ref(),used,unused.into_iter().chain(index));
    }

    /*
    fn match_get_mut<Iter:Iterator<Item = Key>>(&mut self,mut index:Iter)->(Option<&mut Value>,impl Iterator<Item = Key>) {
        let mut used=Vec::<Key>::new();
        let mut current: &mut IndexTree<Key, Value>=self;
        let mut selected:Option<RefCell<&mut IndexTree<Key, Value>>>=None;

        loop {
            let next_key_=index.next();
            if let Some(next_key)=next_key_{
                let next_=current.nexts.get_mut(&next_key);
                used.push(next_key);
                if let Some(next)=next_{
                    if next.value.is_some(){
                        selected=Some(RefCell::new(next));
                        used.clear();
                    }else {
                        
                    }
                    current=next;
                }else {
                    break;
                }
            }else {
                break;
            }
        }
        let return_iter=used.into_iter().chain(index);
        if let Some(mut sel)=selected{
            let dwa=sel.get_mut();
            todo!();
            return (dwa.value.as_mut(),return_iter);
        }else {
            return (None,return_iter);
        }
    } */
}

impl<Key,Value,const N:usize> From<[( [Key;N],Value);N]> for Trie<Key,Value> 
    where Key:Hash+Eq
{
    fn from(arr: [( [Key;N],Value);N]) -> Self {
        let mut v=Self::new();
        for i in arr {
            v.insert(i.0.into_iter(), i.1);
        }
        v
    }
    
}

impl<Key,Value,KeyIter> FromIterator<(KeyIter,Value)> for Trie<Key,Value> 
    where Key:Hash+Eq,
    KeyIter:IntoIterator<Item = Key>
{
    fn from_iter<T: IntoIterator<Item = (KeyIter,Value)>>(iter: T) -> Self {
        let mut v=Self::new();
        for i in iter {
            v.insert(i.0, i.1);
        }
        v
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test(){
        let str1=String::from("aaa");
        let str2=String::from("aaabb");
        let str3=String::from("aabbb");

        let mut index_tree=Trie::new();
        index_tree.insert(str1.chars(), 1);
        index_tree.insert(str2.chars(), 2);
        assert_eq!(index_tree.get(str1.chars()),Some(&1));
        index_tree.insert(str2.chars(), 3);
        assert_eq!(index_tree.get(str2.chars()),Some(&3));
        assert_eq!(index_tree.get(str3.chars()),None);

        let teststr=String::from("aaabb aabbb");
        let res=index_tree.match_get(teststr.chars());
        assert_eq!(res.0,Some(&3));
        let res1_collect:Vec<char>=res.2.collect();
        assert_eq!(res1_collect,String::from(" aabbb").chars().collect::<Vec<char>>());
        
    }
}