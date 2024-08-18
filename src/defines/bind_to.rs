use super::factory::Factory;

pub trait BindTo<CF,C>
    where CF:Factory<T = C>+?Sized
{
    type T;
    fn bind_to<F>(cf:&mut CF,f:F)->Self
        where F:Fn(&C)->&mut Self::T;
}

pub trait Container<C>
{
    
}