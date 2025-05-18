use std::{collections::HashMap, hash::Hash};




pub struct SortByOrder{
    nodes:Vec<SortByOrderNode>,
    res:Vec<usize>,
    updated:bool
}
struct SortByOrderNode{
    pub befores:Vec<usize>
}

impl SortByOrder {
    pub fn new()->Self {
        SortByOrder{
            nodes:Vec::new(),
            res:Vec::new(),
            updated:false
        }
    }

    pub fn add_item(&mut self)->usize{
        let id=self.nodes.len();
        self.nodes.push(SortByOrderNode::new());
        self.updated=false;
        id
    }

    pub fn add_order(&mut self, a:usize,b:usize){
        self.nodes[b].befores.push(a);
        self.updated=false;
    }
    
    pub fn get_res<'a>(&'a mut self)->&'a[usize] {
        use NodeState::*;
        if !self.updated {
            if self.res.capacity()<self.nodes.len() {
                self.res=Vec::with_capacity(self.nodes.len());
            }
            let mut state_list= Vec::<NodeState>::with_capacity(self.nodes.len());
            fn add_to_res(id:usize, state_list:&mut Vec<NodeState>,res_list:&mut Vec<usize>,nodes:&Vec<SortByOrderNode>){
                match state_list[id] {
                    Unchecked=>{
                        state_list[id]=Checking;
                        for i in &nodes[id].befores {
                            add_to_res(*i,state_list,res_list,nodes);
                        }
                        res_list.push(id);
                        state_list[id]=Checked;
                    },
                    Checking=>{
                        panic!("loop ref");
                    },
                    Checked=>{
                        return;
                    }
                }
            }
            for _ in 0..self.nodes.len(){
                state_list.push(Unchecked);
            }
            for i in 0.. self.nodes.len() {
                add_to_res(i, &mut state_list, &mut self.res, &self. nodes)
            }
            self.updated=true;
        }
        return &self.res;
    }

    pub fn add_orders<'a>(&mut self,order:&'a impl Order<'a, usize>){
        let k=order.key();
        for i in order.befores() {
            self.add_order(*i, *k);
        }
        for i in order.afters() {
            self.add_order(*k, *i)
        }
    }
}

impl IntoIterator for SortByOrder {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.res.into_iter()
    }
}

impl SortByOrderNode {
    pub fn new()->Self{
        SortByOrderNode{
            befores:Vec::new()
        }
    }
}
enum NodeState {
    Unchecked,
    Checking,
    Checked
}

pub struct SortByOrderWithKV<'a,TKey,TValue>
    where TKey:Eq+Hash
{
    sbo:SortByOrder,
    key_to_id_map:HashMap<&'a TKey,usize>,
    id_to_value_list:Vec<Option< TValue>>,
}

impl<'a,TKey:Eq+Hash+'a,TValue> SortByOrderWithKV<'a,TKey,TValue>
{
    pub fn new()->Self {
        SortByOrderWithKV{
            sbo:SortByOrder::new(),
            key_to_id_map:HashMap::new(),
            id_to_value_list:Vec::new()
        }
    }

    pub fn get_or_add_key_to_id(&mut self, key:&'a TKey)->usize
    {
        let r= self.key_to_id_map.get(key);
        match r {
            Some( res)=>return *res,
            None=>{
                let id=self.sbo.add_item();
                self.key_to_id_map.insert(key, id);
                self.id_to_value_list.push(Option::None);
                return  id;
            }
        }
    }

    pub fn id_to_value(&mut self,id:usize)->&mut Option<TValue>{
        return &mut self.id_to_value_list[id];
    }

    pub fn insert(&mut self,key:&'a TKey,value:TValue)
    {
        let id=self.get_or_add_key_to_id(key);
        self.id_to_value_list[id]=Some(value);
    }

    pub fn add_order(&mut self,a:&'a TKey,b:&'a TKey){
        let aid=self.get_or_add_key_to_id(a);
        let bid=self.get_or_add_key_to_id(b);
        self.sbo.add_order(aid,bid);
    }

    pub fn get_res<'b>(&'b mut self)->impl Iterator<Item = &'b TValue>{
        let res= self.sbo.get_res() .iter().filter_map( |id:& 'b usize| {
            match &self.id_to_value_list[*id] {
                Some(v)=>Some(v),
                None=>None
            }
        } );
        return  res;
    }

    pub fn add_orders(&mut self,order:&'a impl Order<'a,TKey>){
        let k=self.get_or_add_key_to_id( order.key());

        for i in order.befores() {
            let i_=self.get_or_add_key_to_id(i);
            self.sbo.add_order(i_, k);
        }
        for i in order.afters() {
            let i_=self.get_or_add_key_to_id(i);
            self.sbo.add_order(k, i_)
        }
    }
}

pub trait Order<'a,K:Eq+Hash> {
    fn key(&'a self)->&'a K;
    fn befores<'b>(&'b self)->impl Iterator<Item=&'a K> +'b
        where K:'a;
    fn afters<'b>(&'b self)->impl Iterator<Item=&'a K>+'b
        where K:'a;
}


pub struct OrderSymVal<'a,K:Eq+Hash>{
    pub key:&'a K,
    pub befores:Option< Vec<&'a K>>,
    pub afters:Option< Vec<&'a K>>
}
impl<'a,K:Eq+Hash> Order<'a,K> for OrderSymVal<'a,K> {
    fn key(&self)->&'a K {
        self.key
    }
    fn befores<'b>(&'b self)->impl Iterator<Item=&'a K> +'b
    {
        let deref_fn=|x:& &'a K| *x;
        match &self.befores {
            Some(v)=>return v.iter().map(deref_fn),
            None=>return [].iter().map(deref_fn)//EmptyIter::new()
        } 
    }

    fn afters<'b>(&'b self)->impl Iterator<Item=&'a K>+'b
            where K:'a {

        let deref_fn=|x:& &'a K| *x;
        match &self.afters {
            Some(v)=>v.iter().map(deref_fn),
            None=>[].iter().map(deref_fn)
        }
    }
}
