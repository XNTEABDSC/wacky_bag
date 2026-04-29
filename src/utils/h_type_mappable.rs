use frunk::{HCons, HNil, Poly};

use crate::utils::type_fn::TypeFunc;




pub trait HTypeMappable<Mapper> {
    type Output;
}


impl<F> HTypeMappable<F> for HNil {
    type Output = HNil;
}

impl<F, R, H, Tail> HTypeMappable<F> for HCons<H, Tail>
where
    F: Fn(H) -> R,
    Tail: HTypeMappable<F>,
{
    type Output = HCons<R, <Tail as HTypeMappable<F>>::Output>;
}

impl<F, R, MapperTail, H, Tail> HTypeMappable<HCons<F, MapperTail>> for HCons<H, Tail>
where
    F: FnOnce(H) -> R,
    Tail: HTypeMappable<MapperTail>,
{
    type Output = HCons<R, <Tail as HTypeMappable<MapperTail>>::Output>;
}

impl<P, H, Tail> HTypeMappable<Poly<P>> for HCons<H, Tail>
where
    P: TypeFunc<H>,
    Tail: HTypeMappable<Poly<P>>,
{
    type Output = HCons<<P as TypeFunc<H>>::Output, <Tail as HTypeMappable<Poly<P>>>::Output>;
}