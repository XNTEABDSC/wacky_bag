


pub trait RangeStep
    where for<'a> &'a Self:IntoIterator<Item =  Self::Item>
{
    type Item;
    fn step_between(a:&Self::Item,b:&Self::Item)->isize;
    fn step_from_start(a:&Self::Item)->isize;
    fn start()->Self::Item;
    fn last()->Self::Item;
    fn next(a:&Self::Item)->Option<Self::Item>;
    fn next_assign(a:&mut Self::Item);
}