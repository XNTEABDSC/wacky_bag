
use std::{collections::HashMap, hash::Hash, iter::Chain, mem};

pub struct Trie<Key,TValue>
    where Key:Hash+Eq
{
    value:Option<TValue>,
    nexts:HashMap<Key,Trie<Key,TValue>>
}

unsafe impl<Key: Sync, TValue: Sync> Sync for Trie<Key, TValue>
where Key:Hash+Eq
{
}

impl<Key,Value> Trie<Key,Value> 
    where Key:Hash+Eq
{
    pub fn new()->Self {
        Self { nexts: Default::default(),value:None }
    }

    pub fn travel(&self,mut index:impl Iterator<Item = Key>)->Option<&Trie<Key,Value>> {
        let key_=index.next();
        if let Some(key)=key_{
            self.nexts.get(&key).map_or(None,|next|next.travel(index))
        }else {
            Some(self)
        }
    }

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

    pub fn insert(&mut self,mut index:impl Iterator<Item = Key>,value:Value)->Option<Value>{
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

    pub fn get(&self,index:impl Iterator<Item = Key>)->Option<&Value> {
        self.travel(index).map_or(None,|n|n.value.as_ref())
    }

    pub fn get_mut(&mut self,index:impl Iterator<Item = Key>)->Option<&mut Value> {
        self.travel_mut(index).map_or(None,|n|n.value.as_mut())
    }

    pub fn is_empty(&self)->bool{
        return  self.value.is_none()&&self.nexts.len()==0;
    }


    pub fn match_get<Iter:Iterator<Item = Key>>(&self,mut index:Iter)->(Option<&Value>,Chain< <Vec<Key> as IntoIterator >::IntoIter , Iter>) {
        let mut used=Vec::<Key>::new();
        let mut current: &Trie<Key, Value>=self;
        let mut selected:&Trie<Key, Value>=self;

        loop {
            let next_key_=index.next();
            if let Some(next_key)=next_key_{
                let next_=current.nexts.get(&next_key);
                used.push(next_key);
                if let Some(next)=next_{
                    if next.value.is_some(){
                        selected=next;
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
        return (selected.value.as_ref(),used.into_iter().chain(index));
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
    KeyIter:Iterator<Item = Key>
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
        let res1_collect:Vec<char>=res.1.collect();
        assert_eq!(res1_collect,String::from(" aabbb").chars().collect::<Vec<char>>());
        
    }
}