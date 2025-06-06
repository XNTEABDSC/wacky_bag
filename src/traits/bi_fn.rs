use std::marker::PhantomData;


pub trait BiFn<A,B>
{
    fn func(&self,a:A)->B;
    fn inv_func(&self,b:B)->A;
}

pub struct BiFnby2<Func,InvFunc>
{   pub func_:Func,
    pub inv_func_:InvFunc,
}

impl<A,B,Func,InvFunc> BiFn<A,B> for BiFnby2<Func,InvFunc>
where Func:Fn(A)->B,
InvFunc:Fn(B)->A
{
    fn func(&self,a:A)->B {
        (self.func_)(a)
    }

    fn inv_func(&self,b:B)->A {
        (self.inv_func_)(b)
    }
}