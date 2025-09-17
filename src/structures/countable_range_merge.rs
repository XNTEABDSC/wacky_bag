use std::{array, iter::{Map, Rev}, ops::{ControlFlow, Deref}};

use crate::{structures::{just::Just, typed_deref::NTDeref}, traits::countable_range::{CountableRangeForRef, CountableRangeItem, CountableRangeItemN}};


#[derive(Clone, Copy)]
pub struct CountableRangeMerge2<ARange,BRange>
{
    a:ARange,
    b:BRange,
}

#[derive(Clone, Copy)]
pub struct CountableRangeMerge2Item<AItem,BItem>{
    a_item:AItem,
    b_item:BItem,
}

impl<AItem, BItem> CountableRangeItemN for CountableRangeMerge2Item<AItem, BItem> 
    where 
        AItem:CountableRangeItem+CountableRangeItemN,
        BItem:CountableRangeItem+CountableRangeItemN,
{
    fn move_n_assign(&mut self,n:isize)->isize {
        let i=self.b_item.move_n_assign(n);
        if i==0{
            return 0;
        }
        self.a_item.move_n_assign(i)
    }
}
impl<AItem,BItem> CountableRangeMerge2Item<AItem,BItem> {
    pub fn inner(&self)->(&AItem,&BItem) {
        (&self.a_item,&self.b_item)
    }
    pub fn into_inner(self)->(AItem,BItem) {
        (self.a_item,self.b_item)
    }
}
impl<AItem,BItem> CountableRangeItem for CountableRangeMerge2Item<AItem,BItem>
    where 
        AItem:CountableRangeItem,
        BItem:CountableRangeItem,
        //for<'a> AItem::RangeRef<'a>:CountableRangeForRef,
        //for<'a> BItem::RangeRef<'a>:CountableRangeForRef,
{

    fn index(&self)->usize {
        let a_idx=self.a_item.index();
        let b_idx=self.b_item.index();

        return  a_idx*self.b_item.range().index_len()+b_idx;
        //self.a_item.index()*self.b_item.range().index_len()+self.b_item.index()
    }

    fn first_assign(&mut self) {
        self.a_item.first_assign();
        self.b_item.first_assign();
    }
    
    
    fn next_assign(&mut self)->bool {
        if self.b_item.next_assign(){
            if self.a_item.next_assign(){
                return true;
            }
        }
        return false;
    }
    
    type RangeRef<'a> =  CountableRangeMerge2<AItem::RangeRef<'a>,BItem::RangeRef<'a>>  where 
        Self:'a,
        AItem::RangeRef<'a>:CountableRangeForRef,
        BItem::RangeRef<'a>:CountableRangeForRef;
    
    fn range<'a>(&'a self)->Self::RangeRef<'a> {
        CountableRangeMerge2{a:self.a_item.range(),b:self.b_item.range()}
    }
    
    fn prev_assign(&mut self)->bool {
        if self.b_item.prev_assign(){
            if self.a_item.prev_assign(){
                return true;
            }
        }
        return false;
    }
    
    fn last_assign(&mut self) {
        self.a_item.last_assign();
        self.b_item.last_assign();
    }
}

impl<ARange,BRange> CountableRangeForRef for CountableRangeMerge2<ARange,BRange>
    where 
        ARange:CountableRangeForRef,
        BRange:CountableRangeForRef,
{
    type Item=CountableRangeMerge2Item<
        <ARange as CountableRangeForRef>::Item,
        <BRange as CountableRangeForRef>::Item,
    >;

    fn item_first(self)->Self::Item {
        CountableRangeMerge2Item{
            a_item:self.a.item_first(),
            b_item:self.b.item_first(),
        }
    }

    fn item_from_index(self,index:usize)->Option<Self::Item> {
        //if index>=self.index_len(){return None;}
        let index_len=self.b.index_len();
        let (div,rem)=(index/index_len,index%index_len);
        let a_item_may=self.a.item_from_index(div);
        let b_item_may=self.b.item_from_index(rem);
        if let (Some(a_item),Some(b_item))=(a_item_may,b_item_may){
            Some(
                CountableRangeMerge2Item{
                    a_item,
                    b_item
                }
            )
        }else {
            None
        }
        
    }

    fn index_len(&self)->usize {
        self.a.index_len()*self.b.index_len()
    }
    
    fn item_last(self)->Self::Item {
        CountableRangeMerge2Item{
            a_item:self.a.item_last(),
            b_item:self.b.item_last(),
        }
        
    }
}



#[derive(Clone,Copy)]
pub struct CountableRangeMergeArray<TRange,const LEN:usize>
{
    ranges:[TRange;LEN],
    index_len:usize// we knows nothings about how TRange stores, so we should let users decide how max_len stores, but copy is better.
}

impl<TRange, const LEN: usize> CountableRangeMergeArray<TRange, LEN> 
    where TRange:CountableRangeForRef
{
    pub fn new(ranges:[TRange;LEN])->Self {
        let index_len=ranges.iter().fold(1, |acc,range|acc*range.index_len());
        Self{
            ranges,index_len
        }
    }
}
#[derive(Clone,Copy)]
pub struct CountableRangeMergeArrayItem<TItem,const LEN:usize>{
    items:[TItem;LEN],
    cached_index:usize,
    index_len:usize
}

impl<TItem, const LEN: usize> CountableRangeMergeArrayItem<TItem, LEN> 
    where TItem:CountableRangeItem+CountableRangeItemN
{
    
    pub fn move_n_assign_at_dim(&mut self,dim_idx:usize,n:isize)->isize {
        assert!(dim_idx<LEN, "dim_idx out of range");
        match self.items.iter_mut().rev().skip(LEN-dim_idx).try_fold(n,|acc,item|{
            let res=item.move_n_assign(acc);
            if res==0{
                ControlFlow::Break(())
            }else {
                ControlFlow::Continue(res)
            }
        }){
            ControlFlow::Continue(res) => res,
            ControlFlow::Break(_) => 0,
        }
    }
}

impl<TItem, const LEN: usize> CountableRangeMergeArrayItem<TItem, LEN> 
    where TItem: CountableRangeItem
{
    pub fn next_assign_at_dim(&mut self,dim_idx:usize)->bool{
        assert!(dim_idx<LEN, "dim_idx out of range");
        match self.items.iter_mut().rev().skip(LEN-dim_idx).try_for_each(|item|{
            if item.next_assign() {
                return ControlFlow::Continue(());
            } else {
                return ControlFlow::Break(());
            }
        }){
            ControlFlow::Continue(_) => {
                self.cached_index=0;
                return true;

            },
            ControlFlow::Break(_) => {
                self.cached_index+=1;
                return false;
            },
        }
    }
    pub fn prev_assign_at_dim(&mut self,dim_idx:usize)->bool{
        assert!(dim_idx<LEN, "dim_idx out of range");
        match self.items.iter_mut().rev().skip(LEN-dim_idx).try_for_each(|item|{
            if item.prev_assign() {
                return ControlFlow::Continue(());
            } else {
                return ControlFlow::Break(());
            }
        }){
            ControlFlow::Continue(_) => {
                self.cached_index=0;
                return true;

            },
            ControlFlow::Break(_) => {
                self.cached_index-=1;
                return false;
            },
        }
    }
    pub fn next_assign_at_dim_loop(&mut self,dim_idx:usize)->bool {
        self.items[dim_idx].next_assign()
    }
    pub fn prev_assign_at_dim_loop(&mut self,dim_idx:usize)->bool {
        self.items[dim_idx].prev_assign()
    }
}

impl<TItem, const LEN: usize> CountableRangeMergeArrayItem<TItem, LEN> {
    pub fn into_inner(self)->([TItem;LEN],usize,usize) {
        (self.items,self.cached_index,self.index_len)
    }
}



impl<TItem, const LEN: usize> CountableRangeItemN for CountableRangeMergeArrayItem<TItem, LEN> 
    where TItem: CountableRangeItem+CountableRangeItemN 
{
    fn move_n_assign(&mut self,n:isize)->isize {
        self.cached_index+=n.rem_euclid(self.index_len as isize) as usize;
        // #[cfg(debug_assertions)]
        // let mut div=n.div_euclid(self.index_len as isize);
        if self.cached_index>self.index_len{
            self.cached_index-=self.index_len;
            // #[cfg(debug_assertions)]
            // div=div+1;
        }

        let result=self.items.iter_mut().rev().fold(n,|n,i|i.move_n_assign(n));
        //debug_assert_eq!(div,result);
        result
    }
}

impl<TItem,const LEN:usize> Deref for CountableRangeMergeArrayItem<TItem,LEN> {
    type Target=[TItem;LEN];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<TItem,const LEN:usize> CountableRangeItem for CountableRangeMergeArrayItem<TItem,LEN> 
    where 
        TItem:CountableRangeItem,
{
    type RangeRef<'a> = CountableRangeMergeArray< TItem::RangeRef<'a>,LEN >
        where 
        Self:'a,;

    fn next_assign(&mut self)->bool {
        let res=self.items.iter_mut().rev().try_for_each(|item|{
            if item.next_assign() {
                return ControlFlow::Continue(());
            } else {
                return ControlFlow::Break(());
            }
        });
        match res {
            ControlFlow::Continue(_) => {
                self.cached_index=0;
                return true;

            },
            ControlFlow::Break(_) => {
                self.cached_index+=1;
                return false;
            },
        }
    }

    fn prev_assign(&mut self)->bool {
        
        let res=self.items.iter_mut().rev().try_for_each(|item|{
            if item.prev_assign() {
                return ControlFlow::Continue(());
            } else {
                return ControlFlow::Break(());
            }
        });
        match res {
            ControlFlow::Continue(_) => {
                self.cached_index=0;
                return true;

            },
            ControlFlow::Break(_) => {
                self.cached_index-=1;
                return false;
            },
        }
    }

    fn first_assign(&mut self) {
        self.items.iter_mut().for_each(|i|i.first_assign());
        self.cached_index=0;
    }

    fn last_assign(&mut self) {
        self.items.iter_mut().for_each(|i|i.last_assign());
        self.cached_index=self.index_len-1;
    }

    fn index(&self)->usize {
        debug_assert_eq!(
            self.cached_index,
            self.items.iter().fold(0, |acc,item|{
                acc*item.range().index_len()+item.index()
            })
        );
        self.cached_index
    }

    fn range<'a>(&'a self)->Self::RangeRef<'a> {
        CountableRangeMergeArray{
            ranges:array::from_fn(|i|self.items[i].range()),
            index_len:self.index_len
        }
    }
}

impl<TRange,const LEN:usize> CountableRangeForRef for CountableRangeMergeArray<TRange,LEN>
    where TRange:CountableRangeForRef
{
    type Item=CountableRangeMergeArrayItem<TRange::Item,LEN>;

    fn item_first(self)->Self::Item {
        CountableRangeMergeArrayItem{
            items:self.ranges.map(|i|i.item_first()),
            index_len:self.index_len,
            cached_index:0,
        }
    }

    fn item_last(self)->Self::Item {
        CountableRangeMergeArrayItem{
            items:self.ranges.map(|i|i.item_last()),
            index_len:self.index_len,
            cached_index:self.index_len-1,
        }
    }

    fn item_from_index(self,mut index:usize)->Option<Self::Item> {
        let mut range_idxes=self.ranges.map(|a|{(a,0)});

        range_idxes.iter_mut().rev().for_each(|(range,idx)|{
            let i_idx=range.index_len();
            let (div,rem)=(index/i_idx,index%i_idx);
            *idx=rem;
            index=div;

        });

        Some(CountableRangeMergeArrayItem{
            items: range_idxes.map(|a|a.0.item_from_index(a.1).unwrap()),//array::from_fn(|i|self.ranges[i].item_from_index(sub_idxes[i]).unwrap()),
            cached_index: index,
            index_len: self.index_len,
        })
    }

    fn index_len(&self)->usize {
        debug_assert_eq!(
            self.index_len,
            self.ranges.iter().fold(1, |acc,range|acc*range.index_len())
        );
        self.index_len
    }
}

#[cfg(test)]
mod test{
    use std::rc::Rc;

    use crate::{structures::{countable_range_merge::CountableRangeMerge2, countable_range_std::CountableRangeStd, typed_deref::ToNTDeref}, traits::countable_range::{CountableRangeForRef, CountableRangeItem}};

    use super::*;
    #[test]
    fn test_merge2(){
        let idx1=CountableRangeMerge2{
            a:Rc::new(CountableRangeStd::from_range(-3..3)).as_nt_deref(),
            b:Rc::new(CountableRangeStd::from_range(-2..2)).as_nt_deref()};
        let mut idx1_item1=idx1.clone().item_first();
        println!("test next");
        loop {
            let (a,b)=idx1_item1.inner();
            println!("{:?}:{:?}",(a.inner(),b.inner()),idx1_item1.index());
            if idx1_item1.next_assign(){
                break;
            }
        }
        println!("test prev");
        let mut idx1_item2 = idx1.clone().item_last();
        loop {
            let (a,b)=idx1_item2.inner();
            println!("{:?}:{:?}",(a.inner(),b.inner()),idx1_item2.index());
            if idx1_item2.prev_assign(){
                break;
            }
        }
    }
    #[test]
    fn test_merge_array(){
        let idx1=CountableRangeMergeArray::new([
            Rc::new(CountableRangeStd::from_range(0..4)).as_nt_deref(),
            Rc::new(CountableRangeStd::from_range(0..4)).as_nt_deref(),
            Rc::new(CountableRangeStd::from_range(0..4)).as_nt_deref(),
        ]);
        assert_eq!(idx1.index_len(),4*4*4);
        let mut idx1_item=idx1.clone().item_first();
        loop {
            let things=idx1_item.deref();
            println!("{:?}:{}",things.iter().map(|a|a.deref()).collect::<Vec<_>>(),idx1_item.index());
            if idx1_item.next_assign() {
                break;
            }
        }
        let mut idx2_item=idx1.clone().item_first();
        loop {
            let things=idx2_item.deref();
            println!("{:?}:{}",things.iter().map(|a|a.deref()).collect::<Vec<_>>(),idx2_item.index());
            if idx2_item.move_n_assign(3)>0 {
                break;
            }
        }
        let a=1;
        let a_box:Box<_>=a.into();
        let b=1;
        let b_rc:Rc<_>=b.into();
        let c=1;
        let c_ref:Just<_>=c.into();
    }
}