#[macro_export]
macro_rules! derive_linear_ops_dim {
    ($type:ty) => {
        
        impl<const DIM:usize> std::ops::Add for $type<DIM>  {
            type Output=$type<DIM>;

            fn add(self, rhs: Self) -> Self::Output {
                $type(self.0+rhs.0)
            }
        }
        impl<const DIM:usize> std::ops::Sub for $type<DIM>  {
            type Output=$type<DIM>;

            fn sub(self, rhs: Self) -> Self::Output {
                $type(self.0-rhs.0)
            }
        }
        impl<const DIM:usize> std::ops::Neg for $type<DIM>  {
            type Output=$type<DIM>;
            fn neg(self) -> Self::Output {
                $type(-self.0)
            }
        }

        impl<const DIM:usize> std::ops::AddAssign for $type<DIM>  {
            fn add_assign(&mut self, rhs: Self) {
                self.0+=rhs.0
            }
        }
        impl<const DIM:usize> std::ops::SubAssign for $type<DIM>  {
            fn sub_assign(&mut self, rhs: Self) {
                self.0-=rhs.0
            }
        }
    };
}

impl<const DIM:usize> std::ops::Add for Pos<DIM>  {
    type Output=Pos<DIM>;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0+rhs.0)
    }
}
impl<const DIM:usize> std::ops::Sub for Pos<DIM>  {
    type Output=Pos<DIM>;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0-rhs.0)
    }
}
impl<const DIM:usize> std::ops::Neg for Pos<DIM>  {
    type Output=Pos<DIM>;
    fn neg(self) -> Self::Output {
        Pos(-self.0)
    }
}

impl<const DIM:usize> std::ops::AddAssign for Pos<DIM>  {
    fn add_assign(&mut self, rhs: Self) {
        self.0+=rhs.0
    }
}
impl<const DIM:usize> std::ops::SubAssign for Pos<DIM>  {
    fn sub_assign(&mut self, rhs: Self) {
        self.0-=rhs.0
    }
}
impl<const DIM:usize> std::ops::Mul<Num> for Pos<DIM>  {
    type Output=Pos<DIM>;

    fn mul(self, rhs: Num) -> Self::Output {
        Pos(self.0*rhs)
    }
}
impl<const DIM:usize> std::ops::Div<Num> for Pos<DIM>  {
    type Output=Pos<DIM>;

    fn div(self, rhs: Num) -> Self::Output {
        Pos(self.0/rhs)
    }
}
impl<const DIM:usize> std::ops::MulAssign<Num> for Pos<DIM>  {
    fn mul_assign(&mut self, rhs: Num) {
        self.0*=rhs
    }
}
impl<const DIM:usize> std::ops::DivAssign<Num> for Pos<DIM>  {

    fn div_assign(&mut self, rhs: Num) {
        self.0/=rhs
    }
}