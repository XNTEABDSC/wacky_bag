
use std::{array, ops::{self, Index, IndexMut}};


#[derive(Clone, Copy,PartialEq, Eq,Debug)]
pub struct MVec<Num,const N:usize>(pub [Num;N]);

impl<Num,const N:usize> MVec<Num,N> {
    /*
    pub const fn new(arr:[Num;N])->Self {
        MVec(arr)
    } */
}

impl<Num: Default,const N:usize> Default for MVec<Num,N>  {
    fn default() -> Self {
        Self(array::from_fn(|_|{Default::default()}))
    }
}

impl<Num: std::ops::Add<Output = Num>,const N:usize> ops::Add for MVec<Num,N> {
    type Output=MVec<Num,N>;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        
        let newarr=crate::utils::array_utils::array_from_2arr(self.0, rhs.0, |a,b|{a+b});
        /*
        let mut aiter=self.0.into_iter();
        let mut biter=rhs.0.into_iter();
        let newarr=array::from_fn(|i|{
            aiter.next().unwrap()+biter.next().unwrap()
        }); */
        return MVec(newarr);
    }
}

impl<Num: std::ops::AddAssign,const N:usize> ops::AddAssign for MVec<Num,N> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0.iter_mut().zip(rhs.0.into_iter()).for_each(|(a,b)|{*a+=b});
    }
}

impl<Num:ops::Mul<Output = Num>+ops::Add<Output = Num>+Default,const N:usize> ops::Mul for MVec<Num,N> {
    type Output=Num;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.0.into_iter().zip(rhs.0.into_iter()).fold(Default::default(), |acc,(a,b)|{acc+a*b})
    }
}

impl<Num:ops::Mul<Output = Num>+Copy,const N:usize> ops::Mul<Num> for MVec<Num,N> {
    type Output=MVec<Num,N>;

    #[inline]
    fn mul(self, rhs: Num) -> Self::Output {
        MVec(self.0.map(|i|{i*rhs}))
    }
}

impl<Num:ops::MulAssign<Num>+Copy,const N:usize> ops::MulAssign<Num> for MVec<Num,N> {
    #[inline]
    fn mul_assign(&mut self, rhs: Num) {
        for i in self.0.iter_mut() {
            (*i) *=rhs
        }
    }
}

impl<Num:ops::Div<Output = Num>+Copy,const N:usize> ops::Div<Num> for MVec<Num,N> {
    type Output=MVec<Num,N>;

    fn div(self, rhs: Num) -> Self::Output {
        MVec(self.0.map(|i|{i/rhs}))
        
    }
}

impl<Num:ops::DivAssign<Num>+Copy,const N:usize> ops::DivAssign<Num> for MVec<Num,N> {
    fn div_assign(&mut self, rhs: Num) {
        for i in self.0.iter_mut() {
            (*i) /=rhs
        }
    }
}

impl<Num:ops::Neg<Output = Num>,const N:usize> ops::Neg for MVec<Num,N> {
    type Output=MVec<Num,N>;

    #[inline]
    fn neg(self) -> Self::Output {
        MVec(self.0.map(|i|{-i}))
    }
}

impl<Num:ops::Sub<Output = Num>,const N:usize> ops::Sub for MVec<Num,N> {
    type Output=MVec<Num,N>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let newarr=crate::utils::array_utils::array_from_2arr(self.0, rhs.0, |a,b|{a-b});
        /*
        let mut aiter=self.0.into_iter();
        let mut biter=rhs.0.into_iter();
        let newarr=array::from_fn(|i|{
            aiter.next().unwrap()+biter.next().unwrap()
        }); */
        return MVec(newarr);
        
    }
}
impl<Num:ops::SubAssign,const N:usize> ops::SubAssign for MVec<Num,N> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0.iter_mut().zip(rhs.0.into_iter()).for_each(|(a,b)|{*a-=b})
    }
}
impl<Num,const N:usize> Index<usize> for MVec<Num,N>  {
    type Output=Num;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
impl<Num,const N:usize> IndexMut<usize> for MVec<Num,N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
/*
impl<Num> MVec<Num> {
    pub fn rotate90(self)->Self 
        where Num: std::ops::Neg<Output = Num>
    {
        Self(-self.1,self.0)
    }
}
 */
/*
 impl<Num,const N:usize> MVec<Num,N> {
    pub fn exterior_product(arrs:[MVec<Num,N>;N-1]) {
        
    }
 } */