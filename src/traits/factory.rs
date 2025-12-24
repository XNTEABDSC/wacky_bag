
/// self + input -> output
/// you should check and panic if necessary, even if Input is checked via TryFactory, as user may pass InputConfirmed for another's check.
pub trait Factory<Input,Mark>{
    type Output;
    fn make(self,input:Input)->Self::Output;
}

/// check whether we can do self + input -> output
pub trait TryFactory<Input,Mark>:Factory<Self::InputConfirmed,Mark> {
    type InputConfirmed;
    type Err;
    fn can_make(&self,input:Input)->Result<Self::InputConfirmed,Self::Err>;
}

