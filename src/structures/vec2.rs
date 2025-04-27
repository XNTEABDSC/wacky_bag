

use super::mvec::MVec;
pub type Vec2<Num>=MVec<Num,2>;
impl<Num> Vec2<Num> {
    pub const fn new(x:Num,y:Num)->Self {
        MVec([x,y])
    }
}
/*
#[derive(Default,Clone, Copy,PartialEq, Eq,Debug)]
pub struct Vec2<Num>(pub Num,pub Num);

impl<Num> Vec2<Num> {
    pub const fn new(x:Num,y:Num)->Self {
        Vec2(x,y)
    }
}

impl<Num: std::ops::Add<Output = Num>> ops::Add for Vec2<Num> {
    type Output=Vec2<Num>;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0+rhs.0,self.1+rhs.1)
    }
}

impl<Num: std::ops::AddAssign> ops::AddAssign for Vec2<Num> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0+=rhs.0;
        self.1+=rhs.1;
    }
}

impl<Num:ops::Mul<Output = Num>+ops::Add<Output = Num>> ops::Mul for Vec2<Num> {
    type Output=Num;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.0*rhs.0 + self.1*rhs.1
    }
}

impl<Num:ops::Mul<Output = Num>+Copy> ops::Mul<Num> for Vec2<Num> {
    type Output=Vec2<Num>;

    #[inline]
    fn mul(self, rhs: Num) -> Self::Output {
        Vec2(self.0*rhs,self.1*rhs)
    }
}

impl<Num:ops::MulAssign<Num>+Copy> ops::MulAssign<Num> for Vec2<Num> {
    #[inline]
    fn mul_assign(&mut self, rhs: Num) {
        self.0*=rhs;
        self.1*=rhs;
    }
}

impl<Num:ops::Div<Output = Num>+Copy> ops::Div<Num> for Vec2<Num> {
    type Output=Vec2<Num>;

    fn div(self, rhs: Num) -> Self::Output {
        Vec2(self.0/rhs,self.1/rhs)
    }
}

impl<Num:ops::DivAssign<Num>+Copy> ops::DivAssign<Num> for Vec2<Num> {
    fn div_assign(&mut self, rhs: Num) {
        self.0/=rhs;
        self.1/=rhs;
    }
}

impl<Num:ops::Neg<Output = Num>> ops::Neg for Vec2<Num> {
    type Output=Vec2<Num>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec2(-self.0,-self.1)
    }
}

impl<Num:ops::Sub<Output = Num>> ops::Sub for Vec2<Num> {
    type Output=Vec2<Num>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0-rhs.0,self.1-rhs.1)
    }
}
impl<Num:ops::SubAssign> ops::SubAssign for Vec2<Num> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0-=rhs.0;
        self.1-=rhs.1;
    }
}

impl<Num> Vec2<Num> {
    pub fn rotate90(self)->Self 
        where Num: std::ops::Neg<Output = Num>
    {
        Self(-self.1,self.0)
    }
} */