use std::{array, ops::{Add, Index, IndexMut, Mul}};

use crate::structures::cvec::CVec;


#[derive(Debug,Clone, Copy)]
pub struct CMatrix<Num,const DIM0:usize,const DIM1:usize>(pub [[Num;DIM1];DIM0]);

impl<Num,const DIM0:usize,const DIM1:usize> CMatrix<Num,DIM0,DIM1>  {
    pub fn new(values:[[Num;DIM1];DIM0])->Self{Self(values)}

    
}

impl<Num,const DIM0:usize,const DIM1:usize> Default for CMatrix<Num,DIM0,DIM1>  
    where Num:Default
{
    fn default() -> Self {
        Self(array::from_fn(|_|array::from_fn(|_|{Default::default()})))
    }
}

impl<Num,const DIM0:usize,const DIM1:usize> Mul<CVec<Num,DIM1>> for CMatrix<Num,DIM0,DIM1> 
    where Num:Default+Add<Output = Num>+Mul<Output = Num>+Copy
{
    type Output=CVec<Num,DIM0>;

    fn mul(self, rhs: CVec<Num,DIM1>) -> Self::Output 
    {
        CVec::new(
            self.0.map(|m_d0|{
                m_d0.into_iter().zip(rhs.0.clone().into_iter())
                .fold(Num::default(), |acc,x|{
                    acc+x.0*x.1
                })
            })
        )
    }
}

impl<Num,const DIM0:usize,const DIM1:usize> Mul<CMatrix<Num,DIM0,DIM1>> for CVec<Num,DIM0> 
    where Num:Default+Add<Output = Num>+Mul<Output = Num>+Copy
{
    type Output=CVec<Num,DIM1>;

    fn mul(self, rhs: CMatrix<Num,DIM0,DIM1>) -> Self::Output 
    {
        self.0.into_iter().zip(rhs.0.into_iter())
        .map(|v: <std::iter::Zip<array::IntoIter<Num, DIM0>, array::IntoIter<[Num; DIM1], DIM0>> as Iterator>::Item|{
            let (v_d0,m_d0)=v;
            m_d0.map(|m_d0d1|{m_d0d1*v_d0})
            
        }).fold(CVec::new([Num::default();DIM1]), |acc,x|{
            acc+CVec::new(x)
        })
    }
}


impl<Num,const DIM0:usize,const DIM1:usize,const DIM2:usize> Mul<CMatrix<Num,DIM1,DIM2>> for CMatrix<Num,DIM0,DIM1>
    where Num:Default+Add<Output = Num>+Mul<Output = Num>+Copy
{
    type Output = CMatrix<Num,DIM0,DIM2>;

    fn mul(self, rhs: CMatrix<Num,DIM1,DIM2>) -> Self::Output 
    {
        CMatrix::new(
            self.0.map(|m0_d0|{
                (CVec::new(m0_d0)*rhs).0
            })
        )
    }
}

impl<Num,const DIM0:usize,const DIM1:usize> Mul<Num> for CMatrix<Num,DIM0,DIM1>
    where Num:Default+Add<Output = Num>+Mul<Output = Num>+Copy
{
    type Output=Self;

    fn mul(self, rhs: Num) -> Self::Output {
        Self(self.0.map(|a|a.map(|b|b*rhs)))
    }
}


impl<Num,const DIM0:usize,const DIM1:usize> Index<[usize;2]> for CMatrix<Num,DIM0,DIM1> {
    type Output=Num;

    fn index(&self, index: [usize;2]) -> &Self::Output {
        &self.0[index[0]][index[1]]
    }
}

impl<Num,const DIM0:usize,const DIM1:usize> IndexMut<[usize;2]> for CMatrix<Num,DIM0,DIM1> {

    fn index_mut(&mut self, index: [usize;2]) -> &mut Self::Output {
        &mut self.0[index[0]][index[1]]
    }
}