#[macro_export]
macro_rules! derive_add_traits {
    ($T:ty) => {
        

impl std::ops::Add for $T {
    type Output=$T;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        /*
        let mut aiter=self.0.into_iter();
        let mut biter=rhs.0.into_iter();
        let newarr=array::from_fn(|i|{
            aiter.next().unwrap()+biter.next().unwrap()
        }); */
        return $T(self.0+rhs.0);
    }
}

impl std::ops::AddAssign for $T {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0+=rhs.0
    }
}


impl std::ops::Neg for $T {
    type Output=$T;

    #[inline]
    fn neg(self) -> Self::Output {
        $T(-self.0)
    }
}

impl std::ops::Sub for $T {
    type Output=$T;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        /*
        let mut aiter=self.0.into_iter();
        let mut biter=rhs.0.into_iter();
        let newarr=array::from_fn(|i|{
            aiter.next().unwrap()+biter.next().unwrap()
        }); */
        return $T(self.0+rhs.0);
        
    }
}
impl std::ops::SubAssign for $T {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0-=rhs.0
    }
}

    };
}