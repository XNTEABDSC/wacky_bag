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
pub struct CountableRangeMergeArray<TRange,TRList,const LEN:usize>
{
    ranges:[TRange;LEN],
    index_lens:TRList,
    max_len:usize,
    //index_len:usize// we knows nothings about how TRange stores, so we should let users decide how max_len stores, but copy is better.
}

impl<TRange,TRefArray, const LEN: usize> CountableRangeMergeArray<TRange,TRefArray, LEN> 
    where TRange:CountableRangeForRef,
    TRefArray:Deref<Target = [usize;LEN]>,
    [usize;LEN]:Into<TRefArray>,
{
    pub fn new(ranges:[TRange;LEN])->Self {
        let mut index_lens: [usize; LEN]=[0;LEN];
        ranges.iter().zip(index_lens.iter_mut()).rev().fold(1, |mut acc,i|{
            
            *i.1=acc;
            acc*=i.0.index_len();
            
            acc
        });
        let max_len=index_lens[0]*ranges[0].index_len();
        
        //let index_len=ranges.iter().fold(1, |acc,range|acc*range.index_len());
        Self{
            ranges,index_lens:index_lens.into(),max_len
        }
    }
}


impl<TRange,TRefArray, const LEN: usize> CountableRangeMergeArray<TRange,TRefArray, LEN> 
    where TRange:CountableRangeForRef,
    TRefArray:Deref<Target = [usize;LEN]>,
{
    pub unsafe fn from_datas(ranges:[TRange;LEN],index_lens:TRefArray,max_len:usize)->Self{
        Self{ranges,index_lens,max_len}
    }
}
#[derive(Clone,Copy)]
pub struct CountableRangeMergeArrayItem<TItem,TRefArray,const LEN:usize>{
    items:[TItem;LEN],
    cached_index:usize,
    index_lens:TRefArray,
    max_len:usize
}

impl<TItem,TRefArray, const LEN: usize> CountableRangeMergeArrayItem<TItem,TRefArray, LEN> 
    where TItem:CountableRangeItem+CountableRangeItemN,
    TRefArray:Deref<Target = [usize;LEN]>
{
    pub fn move_n_assign_loop_at_dim(&mut self,dim_idx:usize,n:isize)->isize {
        assert!(dim_idx<LEN, "dim_idx out of range");
        let moved=self.items[dim_idx].move_n_assign(n);
        self.cached_index=
            (self.cached_index as isize+(n-moved*self.items[dim_idx].range().index_len() as isize)*self.index_lens[dim_idx] as isize) as usize;
        moved
    }

    pub fn move_n_assign_at_dim(&mut self,dim_idx:usize,n:isize)->isize {
        assert!(dim_idx<LEN, "dim_idx out of range");

        self.cached_index=(self.cached_index as isize + n * self.index_lens[dim_idx] as isize).rem_euclid(self.max_len as isize) as usize ;
        
        let result=match self.items.iter_mut().rev().skip(LEN-dim_idx-1).try_fold(n,|acc,item|{
            if acc==0{
                ControlFlow::Break(())
            }else {
                ControlFlow::Continue(item.move_n_assign(acc))
            }
        }){
            ControlFlow::Continue(res) => res,
            ControlFlow::Break(_) => 0,
        };

        self.check_index();
        result
    }

    
}

impl<TItem, TRefArray, const LEN: usize> CountableRangeMergeArrayItem<TItem, TRefArray, LEN> 
    where TItem: CountableRangeItem,
    TRefArray:Deref<Target = [usize;LEN]>
    {
        fn calc_index(&self)->usize{
        self.items.iter().fold(0, |acc,item|{
            acc*item.range().index_len()+item.index()
        })
    }
    fn check_index(&self){
        debug_assert_eq!(self.cached_index,self.calc_index());
    }

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

impl<TItem, TRefArray, const LEN: usize> CountableRangeMergeArrayItem<TItem, TRefArray, LEN> 

{
    pub fn into_inner(self)->([TItem;LEN],usize,TRefArray,usize) {
        (self.items,self.cached_index,self.index_lens,self.max_len)
    }
}



impl<TItem, TRefArray, const LEN: usize> CountableRangeItemN for CountableRangeMergeArrayItem<TItem, TRefArray, LEN> 
    where TItem: CountableRangeItem+CountableRangeItemN,
        TRefArray:Deref<Target = [usize;LEN]>
{
    fn move_n_assign(&mut self,n:isize)->isize {
        self.cached_index+=n.rem_euclid(self.max_len as isize) as usize;
        // #[cfg(debug_assertions)]
        // let mut div=n.div_euclid(self.index_len as isize);
        if self.cached_index>self.max_len{
            self.cached_index-=self.max_len;
            // #[cfg(debug_assertions)]
            // div=div+1;
        }

        let result=self.items.iter_mut().rev().fold(n,|n,i|i.move_n_assign(n));
        //debug_assert_eq!(div,result);
        result
    }
}

impl<TItem, TRefArray,const LEN:usize> Deref for CountableRangeMergeArrayItem<TItem, TRefArray,LEN> {
    type Target=[TItem;LEN];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<TItem, TRefArray,const LEN:usize> CountableRangeItem for CountableRangeMergeArrayItem<TItem, TRefArray,LEN> 
    where 
        TItem:CountableRangeItem,
        TRefArray:Deref<Target = [usize;LEN]>
{
    type RangeRef<'a> = CountableRangeMergeArray< TItem::RangeRef<'a>,&'a [usize;LEN],LEN >
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
        self.cached_index=self.max_len-1;
    }

    fn index(&self)->usize {
        self.check_index();
        self.cached_index
    }

    fn range<'a>(&'a self)->Self::RangeRef<'a> {
        CountableRangeMergeArray{
            ranges:array::from_fn(|i|self.items[i].range()),
            max_len:self.max_len,
            index_lens:&self.index_lens
        }
    }
}

impl<TRange,TRefArray,const LEN:usize> CountableRangeForRef for CountableRangeMergeArray<TRange,TRefArray,LEN>
    where TRange:CountableRangeForRef,
    TRefArray:Deref<Target = [usize;LEN]>
{
    type Item=CountableRangeMergeArrayItem<TRange::Item,TRefArray,LEN>;

    fn item_first(self)->Self::Item {
        CountableRangeMergeArrayItem{
            items:self.ranges.map(|i|i.item_first()),
            cached_index:0,
            index_lens:self.index_lens,
            max_len:self.max_len,
        }
    }

    fn item_last(self)->Self::Item {
        CountableRangeMergeArrayItem{
            items:self.ranges.map(|i|i.item_last()),
            cached_index:self.max_len-1,
            index_lens:self.index_lens,
            max_len:self.max_len,
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
            index_lens:self.index_lens,
            max_len:self.max_len,
        })
    }

    fn index_len(&self)->usize {
        debug_assert_eq!(
            self.max_len,
            self.ranges.iter().fold(1, |acc,range|acc*range.index_len())
        );
        self.max_len
    }
}

#[cfg(test)]
mod test{
    use std::{process::id, rc::Rc};

    use crate::{structures::{countable_range_merge::CountableRangeMerge2, countable_range_std::CountableRangeStd, typed_deref::ToNTDeref}, traits::countable_range::{CountableRangeForRef, CountableRangeItem}};

    use super::*;
    #[test]
    fn test_merge2(){
        let idx1=CountableRangeMerge2{
            a: CountableRangeStd::new(Rc::new(-3..=2)),//Rc::new(CountableRangeStd::from_range(-3..3)).as_nt_deref(),
            b: CountableRangeStd::new(Rc::new(-3..=2)),//Rc::new(CountableRangeStd::from_range(-2..2)).as_nt_deref()};
        };
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
        let a_range=0..=3;
        let idx1=CountableRangeMergeArray::<_,Just<_>,_>::new([
            CountableRangeStd::new(&a_range),
            CountableRangeStd::new(&a_range),
            CountableRangeStd::new(&a_range),
        ]);
        println!("{:?}",idx1.index_lens);
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
        let mut idx3_item=idx1.clone().item_first();
        loop {
            let things=idx3_item.deref();
            println!("{:?}:{}",things.iter().map(|a|a.deref()).collect::<Vec<_>>(),idx3_item.index());
            if idx3_item.move_n_assign_at_dim(1,3)>0 {
                break;
            }
        }
    }
}