use std::{
    fmt::Debug, marker::PhantomData, ops::{Add, Bound, Deref, Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, Sub}
};

use crate::{structures::{just::Just, typed_deref::{NTDeref, ToNTDeref}}, traits::countable_range::{CountableRangeForRef, CountableRangeItem, CountableRangeItemN}};

// fn check_bound_start<T>(value:&T,bound:Bound<&T>)->bool
//     where T:PartialOrd
// {
//     match bound {
//         Bound::Included(a) => a<=value,
//         Bound::Excluded(a) => a<value,
//         Bound::Unbounded => true,
//     }
// }
// fn check_bound_end<T>(value:&T,bound:Bound<&T>)->bool
//     where T:PartialOrd
// {
//     match bound {
//         Bound::Included(a) => value<=a,
//         Bound::Excluded(a) => value<a,
//         Bound::Unbounded => true,
//     }
// }

fn get_bound_value_ge<T>(bound:Bound<&T>)->Option<T>
    where 
    T:Copy+ Sub<Output=T> + Add<Output = T> + TryInto<usize,Error : Debug>,
    usize: TryInto<T, Error: Debug>,
{
    match bound {
        std::ops::Bound::Included(a) => Some(*a),
        std::ops::Bound::Excluded(a) => Some(*a+(1).try_into().unwrap()),
        std::ops::Bound::Unbounded => None,
    }
}
fn get_bound_value_le<T>(bound:Bound<&T>)->Option<T>
    where 
    T:Copy+ Sub<Output=T> + Add<Output = T> + TryInto<usize,Error : Debug>,
    usize: TryInto<T, Error: Debug>,
{
    match bound {
        std::ops::Bound::Included(a) => Some(*a),
        std::ops::Bound::Excluded(a) => Some(*a-(1).try_into().unwrap()),
        std::ops::Bound::Unbounded => None,
    }
}

pub struct CountableRangeStd<T>
{
    range: RangeInclusive<T>,_p:PhantomData<T>,
    index_len:usize
}

pub struct CountableRangeStdItem<T, RangeRef>
where
    RangeRef: Deref<Target = CountableRangeStd<T>>,
{
    item: T,
    ref_range: RangeRef,
}

impl<T, RangeRef> CountableRangeItemN for CountableRangeStdItem<T, RangeRef>
where
    RangeRef: Deref<Target = CountableRangeStd<T>>,
    T: Copy + Ord + Sub<Output= T> + Add<Output = T> + TryInto<usize, Error: Debug>,
    usize: TryInto<T, Error: Debug>,

{
    fn move_n_assign(&mut self,n:isize)->isize {
        let index_len=self.ref_range.index_len;
        let (mut n_div,n_rem)=(n .div_euclid(index_len as isize), n .rem_euclid(index_len as isize) as usize );
        let mut item_next=self.item+n_rem.try_into().unwrap();
        if &item_next>self.ref_range.range.end(){
            item_next=item_next-index_len.try_into().unwrap();
            n_div+=1;
        }
        self.item=item_next;
        n_div
    }
}

impl<T, RangeRef: std::ops::Deref<Target = CountableRangeStd<T>>> Deref for CountableRangeStdItem<T,RangeRef> {
    type Target=T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T , RangeRef> CountableRangeStdItem<T,RangeRef>
where
    RangeRef: Deref<Target = CountableRangeStd<T>>,
    T:Ord

{
    pub fn inner(&self)->&T {
        &self.item
        // if self.item<self.ref_range.range.end{
        //     Some(&self.item)
        // }else {
        //     None
        // }
    }
    pub fn into_inner(self)->(T,RangeRef){(self.item,self.ref_range)}
}


impl<T, RangeRef> CountableRangeItem for CountableRangeStdItem<T, RangeRef>
where
    RangeRef: Deref<Target = CountableRangeStd<T>>,
    T: Copy + Ord + Sub<Output= T> + Add<Output = T> +TryInto<usize, Error: Debug>,
    usize: TryInto<T, Error: Debug>,
{

    fn index(&self) -> usize {
        let item=self.item;
        let left=*self.ref_range.range.start();
        (item-left).try_into().unwrap()
    }
    
    //type RangeRef<'a>=NTDeref<&'a CountableRangeStd<T>, CountableRangeStd<T>> where T: 'a, RangeRef: 'a;
    
    fn next_assign(&mut self)->bool {
        let item=self.item;
        let v_next = item + (1.try_into().unwrap());
        if &v_next<=self.ref_range.range.end(){
            self.item=v_next;
            return false;
        }
        else {
            self.item=*self.ref_range.range.start();
            return true;
        }
    }
    
    fn prev_assign(&mut self)->bool {
        let item=self.item;
        let v_next = item - (1.try_into().unwrap());
        if self.ref_range.range.start()<=&v_next{
            self.item=v_next;
            return false;
        }
        else {
            self.item=*self.ref_range.range.end();
            return true;
        }
    }
    
    type RangeRef<'a>=NTDeref<&'a CountableRangeStd<T>,CountableRangeStd<T>> where RangeRef: 'a, T: 'a;
    
    fn range<'a>(&'a self)->Self::RangeRef<'a> {
        //&self.ref_range.as_nt_deref()
        NTDeref::new(&self.ref_range)
    }
    
    fn last_assign(&mut self) {
        self.item = *self.ref_range.range.end();
    }

    fn first_assign(&mut self) {
        self.item = *self.ref_range.range.start();//Some(self.ref_range.range.start);
    }
    
    // fn range<'a>(&'a self)->Self::RangeRef<'a> {
    //     (*self.ref_range).as_nt_deref()
    // }
    
}


impl<T, RangeRef> CountableRangeForRef for NTDeref<RangeRef,CountableRangeStd<T>>
//CountableRangeStd<T,RangeRef,ToNewRef>
where
    RangeRef: Deref<Target = CountableRangeStd<T>>,
    T: Copy + Ord + Sub<Output=T> + Add<Output = T> + TryInto<usize,Error : Debug>,
    usize: TryInto<T, Error: Debug>,
{
    type Item = CountableRangeStdItem<T, RangeRef>;

    fn item_first(self) -> Self::Item {
        CountableRangeStdItem {
            item: *self.0.range.start(),//Some(self.0.range.start),
            ref_range: self.0, //.clone()
        }
        //RangeBounds
    }
    fn item_last(self)->Self::Item {
        CountableRangeStdItem {
            item: *self.0.range.end(),
            ref_range: self.0, //.clone()
        }
    }

    fn item_from_index(self, index: usize) -> Option<Self::Item> {
        let real_idx = index.try_into().unwrap() + *self.0.range.start();
        if real_idx > *self.0.range.end() {
            None
        } else {
            Some(CountableRangeStdItem {
                item: real_idx,
                ref_range: self.0, //.clone()
            })
        }
    }

    fn index_len(&self) -> usize {
        (*self.0.range.end() - *self.0.range.start() + (1).try_into().unwrap()).try_into().unwrap()
    }
    
}

impl<T> CountableRangeStd<T> 
    where 
        T: Copy + Ord + Sub<Output=T> + Add<Output = T> + TryInto<usize,Error : Debug>,
        usize: TryInto<T, Error: Debug>,
{
    pub fn new(range:RangeInclusive<T>)->Self {
        Self { _p: Default::default() ,
            index_len:(*range.end() - *range.start() + (1).try_into().unwrap()).try_into().unwrap(),
            range
        }
    }

    pub fn from_range<TRange>(range:TRange)->Self 
        where TRange:RangeBounds<T>
    {
        Self::new(get_bound_value_ge(range.start_bound()).unwrap_or_else(||panic!("range.start_bound() should be bounded"))
            ..=
            get_bound_value_le(range.end_bound()).unwrap_or_else(||panic!("range.end_bound() should be bounded")))
    }
}

struct Meme;
struct MemeItem;
impl CountableRangeItem for MemeItem {

    
    type RangeRef<'a>=NTDeref<&'a Meme, Meme>;
    
    fn next_assign(&mut self)->bool {
        todo!()
    }
    
    fn prev_assign(&mut self)->bool {
        todo!()
    }
    
    fn first_assign(&mut self) {
        todo!()
    }
    
    fn last_assign(&mut self) {
        todo!()
    }
    
    
    fn range<'a>(&'a self)->Self::RangeRef<'a> {
        todo!()
    }
    
    fn index(&self)->usize {
        todo!()
    }
    
}
impl<RangeRef> CountableRangeForRef for NTDeref<RangeRef,Meme>
    where 
        RangeRef: Deref<Target = Meme>,
{
    type Item=MemeItem;
    
    fn item_first(self)->Self::Item {
        todo!()
    }
    
    fn item_last(self)->Self::Item {
        todo!()
    }
    
    fn item_from_index(self,_index:usize)->Option<Self::Item> {
        todo!()
    }
    
    fn index_len(&self)->usize {
        todo!()
    }

}



#[cfg(test)]
mod test {
    use crate::structures::typed_deref::ToNTDeref;

    use super::*;
    use std::rc::Rc;

    #[test]
    fn test() {
        let a_isize_crange = CountableRangeStd::from_range(-2..=1);
        //let _clone_test=a_isize_crange;
        fn check_item<RangeRef: Deref<Target = CountableRangeStd<i32>>>(
            mut a_isize_item: CountableRangeStdItem<i32, RangeRef>,
        ) {
            assert_eq!(a_isize_item.inner(), &-2);
            a_isize_item.next_assign();
            assert_eq!(a_isize_item.inner(), &-1);
            a_isize_item.next_assign();
            assert_eq!(a_isize_item.inner(), &0);
            a_isize_item.next_assign();
            assert_eq!(a_isize_item.inner(), &1);
            assert_eq!(a_isize_item.next_assign(),false);
            assert_eq!(a_isize_item.inner(), &-2);

            a_isize_item.last_assign();
            
            assert_eq!(a_isize_item.inner(), &1);
            a_isize_item.prev_assign();
            assert_eq!(a_isize_item.inner(), &0);
            a_isize_item.prev_assign();
            assert_eq!(a_isize_item.inner(), &-1);
            a_isize_item.prev_assign();
            assert_eq!(a_isize_item.inner(), &-2);
            a_isize_item.prev_assign();
            assert_eq!(a_isize_item.prev_assign(),false);
            assert_eq!(a_isize_item.inner(), &1);


            // assert_eq!(*a_isize_item, -2);
            // a_isize_item = a_isize_item.next().unwrap();
            // assert_eq!(*a_isize_item, -1);
            // a_isize_item = a_isize_item.next().unwrap();
            // assert_eq!(*a_isize_item, -0);
            // a_isize_item = a_isize_item.next().unwrap();
            // assert_eq!(*a_isize_item, 1);
            // let a_isize_item_iter_none = a_isize_item.next();
            // assert!(a_isize_item_iter_none.is_none());
        }
        let a_isize_item = (&a_isize_crange).as_nt_deref().item_first();
        check_item(a_isize_item);
        let c_isize_item;
        {

            let b_isize_crange = Rc::new(a_isize_crange);
            let b_isize_item = (b_isize_crange.clone().as_nt_deref()).item_first();
            c_isize_item = (b_isize_crange.as_nt_deref()).clone().item_first();
            check_item(b_isize_item);
        }
        check_item(c_isize_item);
    }
}
