
/// `self.iter()`
pub trait AsIterator {
    /// return of `self.iter()`
    type Iter<'a>:Iterator<Item = Self::Item<'a>>
    where Self: 'a;
    /// ref item
    type Item<'a>
    where Self: 'a;
    /// iterator which iters ref items
    fn iter<'a>(&'a self)->Self::Iter<'a>;
}

impl<T> AsIterator for T 
where for<'a> &'a T:IntoIterator
{
    type Item<'a> = <&'a T as IntoIterator>::Item
    where Self: 'a;

    type Iter<'a> = <&'a T as IntoIterator>::IntoIter
    where Self: 'a;

    fn iter<'a>(&'a self)->Self::Iter<'a> {
        self.into_iter()
    }
}
