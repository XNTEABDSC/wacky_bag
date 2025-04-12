
pub trait NewTypeOf{
    type Value;
    fn unwrap(&self)->Self::Value;
    fn unwrap_mut(&mut self)->&mut Self::Value;
    fn into_unwrap(self)->Self::Value;
    fn from(value:Self::Value)->Self;
}
