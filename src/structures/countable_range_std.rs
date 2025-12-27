use std::{
    fmt::Debug, marker::PhantomData, ops::{Add, Bound, Deref, RangeBounds, RangeInclusive, Sub}
};

use crate::{structures::{just::Just, typed_deref::NTDeref}, traits::{countable_range::{CountableRangeForRef, CountableRangeItem, CountableRangeItemN}, factory::{Factory, TryFactory}}};



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

#[derive(Debug,Clone, Copy)]
pub struct CountableRangeStd<T,TRange>
    //where TRange:Deref<Target = RangeInclusive<T>>
{
    range: TRange, _p:PhantomData<T>,
    index_len:usize
}

impl<T, TRange> CountableRangeStd<T, TRange> {
    pub fn into_inner(self)->(TRange,usize) {
        (self.range,self.index_len)
    }
}
#[derive(Clone, Copy)]
pub struct CountableRangeStdItem<T,TRange>
where
    //RangeRef: Deref<Target = CountableRangeStd<T,TRange>>,
    //TRange:Deref<Target = RangeInclusive<T>>
{
    item: T,
    ref_range: CountableRangeStd<T,TRange>,
}

impl<T, TRange> CountableRangeItemN for CountableRangeStdItem<T, TRange>
where
    T: Copy + Ord + Sub<Output= T> + Add<Output = T> + TryInto<usize, Error: Debug>,
    usize: TryInto<T, Error: Debug>,
    TRange:Deref<Target = RangeInclusive<T>>

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

impl<T, TRange> Deref for CountableRangeStdItem<T,TRange>
    //where TRange:Deref<Target = RangeInclusive<T>>
{
    type Target=T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T ,TRange> CountableRangeStdItem<T,TRange>
where
    // TRange:Deref<Target = RangeInclusive<T>>,
    // T:Ord

{
    pub fn inner(&self)->&T {
        &self.item
        // if self.item<self.ref_range.range.end{
        //     Some(&self.item)
        // }else {
        //     None
        // }
    }
    pub fn into_inner(self)->(T,CountableRangeStd<T,TRange>){(self.item,self.ref_range)}
}


impl<T, TRange> CountableRangeItem for CountableRangeStdItem<T, TRange>
where
    T: Copy + Ord + Sub<Output= T> + Add<Output = T> +TryInto<usize, Error: Debug>,
    TRange:Deref<Target = RangeInclusive<T>>,
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
    
    type RangeRef<'a>=CountableRangeStd<T, &'a RangeInclusive<T>> where TRange: 'a, T: 'a;
    
    
    fn last_assign(&mut self) {
        self.item = *self.ref_range.range.end();
    }

    fn first_assign(&mut self) {
        self.item = *self.ref_range.range.start();//Some(self.ref_range.range.start);
    }
    
    fn range<'a>(&'a self)->Self::RangeRef<'a> {
        CountableRangeStd{
            range:&self.ref_range.range,
            _p:Default::default(),
            index_len:self.ref_range.index_len
        }
    }
    
}


impl<T, TRange> CountableRangeForRef for CountableRangeStd<T,TRange>
//CountableRangeStd<T,RangeRef,ToNewRef>
where
    T: Copy + Ord + Sub<Output=T> + Add<Output = T> + TryInto<usize,Error : Debug>,
    TRange:Deref<Target = RangeInclusive<T>>,
    usize: TryInto<T, Error: Debug>,
{
    type Item = CountableRangeStdItem<T, TRange>;

    fn item_first(self) -> Self::Item {
        CountableRangeStdItem {
            item: *self.range.start(),//Some(self.0.range.start),
            ref_range: self, //.clone()
        }
        //RangeBounds
    }
    fn item_last(self)->Self::Item {
        CountableRangeStdItem {
            item: *self.range.end(),
            ref_range: self, //.clone()
        }
    }

    fn item_from_index(self, index: usize) -> Option<Self::Item> {
        let real_idx = index.try_into().unwrap() + *self.range.start();
        if real_idx > *self.range.end() {
            None
        } else {
            Some(CountableRangeStdItem {
                item: real_idx,
                ref_range: self, //.clone()
            })
        }
    }

    fn index_len(&self) -> usize {
        (*self.range.end() - *self.range.start() + (1).try_into().unwrap()).try_into().unwrap()
    }
    
}

impl<T, TRange> Factory<T> for CountableRangeStd<T, TRange>
    where 
    T: Copy + Ord + Sub<Output=T> + Add<Output = T> + TryInto<usize,Error : Debug>,
    TRange:Deref<Target = RangeInclusive<T>>,
    usize: TryInto<T, Error: Debug>,
{
    type Output=CountableRangeStdItem<T, TRange>;

    fn make(self,input:T)->Self::Output {
        assert!(self.range.contains(&input));
        CountableRangeStdItem {
            item: input,
            ref_range: self, //.clone()
        }
    }
}

impl<T,TRange> TryFactory<T> for CountableRangeStd<T, TRange>
    where 
    T: Copy + Ord + Sub<Output=T> + Add<Output = T> + TryInto<usize,Error : Debug>,
    TRange:Deref<Target = RangeInclusive<T>>,
    usize: TryInto<T, Error: Debug>,
{
    type InputConfirmed=T;

    type Err=();

    fn can_make(&self,input:T)->Result<Self::InputConfirmed,Self::Err> {
        match self.range.contains(&input) {
            true => Ok(input),
            false => Err(()),
        }
    }
}

impl<T,TRange> CountableRangeStd<T,TRange> 
    where 
        TRange:Deref<Target = RangeInclusive<T>>,
        T: Copy + Ord + Sub<Output=T> + Add<Output = T> + TryInto<usize,Error : Debug>,
        usize: TryInto<T, Error: Debug>,
{
    pub fn new(range:TRange)->Self {
        Self { _p: Default::default() ,
            index_len:(*range.end() - *range.start() + (1).try_into().unwrap()).try_into().unwrap(),
            range
        }
    }

    pub fn from_range<TRange2>(range:TRange2)->Self 
        where TRange2:RangeBounds<T>,
            RangeInclusive<T>:Into<TRange>
    {
        Self::new(
            (get_bound_value_ge(range.start_bound()).unwrap_or_else(||panic!("range.start_bound() should be bounded"))
            ..=
            get_bound_value_le(range.end_bound()).unwrap_or_else(||panic!("range.end_bound() should be bounded"))).into()
        )
    }
}




#[cfg(test)]
mod test {

    use super::*;
    use std::rc::Rc;

    #[test]
    fn test() {
        let a_isize_crange = CountableRangeStd::<_,Just<_>>::from_range(-2..=1);
        //let _clone_test=a_isize_crange;
        fn check_item<TItem>(mut a_isize_item: TItem,)
            where TItem:CountableRangeItem+Deref<Target = i32>
        {
            assert_eq!(a_isize_item.deref(), &-2);
            a_isize_item.next_assign();
            assert_eq!(a_isize_item.deref(), &-1);
            a_isize_item.next_assign();
            assert_eq!(a_isize_item.deref(), &0);
            a_isize_item.next_assign();
            assert_eq!(a_isize_item.deref(), &1);
            assert_eq!(a_isize_item.next_assign(),false);
            assert_eq!(a_isize_item.deref(), &-2);

            a_isize_item.last_assign();
            
            assert_eq!(a_isize_item.deref(), &1);
            a_isize_item.prev_assign();
            assert_eq!(a_isize_item.deref(), &0);
            a_isize_item.prev_assign();
            assert_eq!(a_isize_item.deref(), &-1);
            a_isize_item.prev_assign();
            assert_eq!(a_isize_item.deref(), &-2);
            a_isize_item.prev_assign();
            assert_eq!(a_isize_item.prev_assign(),false);
            assert_eq!(a_isize_item.deref(), &1);


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
        let a_isize_item = a_isize_crange.clone().item_first();
        check_item(a_isize_item);
        let range=Rc::new( a_isize_crange.into_inner().0.0 );

        let a_b=Box::new(String::from("value"));
        let a_b_c=*a_b;
        //let a_b_d=*a_b;

        let c_isize_item;
        {

            let b_isize_crange = CountableRangeStd::new(range);
            let b_isize_item = (b_isize_crange.clone()).item_first();
            c_isize_item = (b_isize_crange).clone().item_first();
            check_item(b_isize_item);
        }
        check_item(c_isize_item);
    }
}
