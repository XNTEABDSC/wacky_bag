use std::{cell::RefCell, mem, ops::{AddAssign, Neg}, sync::Mutex, thread::LocalKey};

#[derive(Debug)]
pub struct AState<TValue,TDelta>
where 
    TDelta:Default+AddAssign<TDelta>,
    TValue:AddAssign<TDelta>
{
    value:TValue,
    delta:Mutex<TDelta>,
}

impl<TValue,TDelta:Default> AState<TValue,TDelta> 
    where 
    TDelta:Default+AddAssign<TDelta>,
    TValue:AddAssign<TDelta>
{
    pub fn new(value:TValue)->Self{
        Self { value, delta: Default::default() }
    }
    pub fn unwrap(&self)->&TValue {
        &self.value
    }
    pub fn add_delta(&self,delta:TDelta)->&Self {
        //*self.delta.borrow_mut()+=delta;
        loop {
            let b=self.delta.try_lock();
            match b {
                Ok(mut v) => {*v+=delta;drop(v);break;},
                Err(_) => {},
            }
        }
        self
    }
    pub fn apply_delta(&mut self)->&Self {
        self.value+=mem::replace(&mut self.delta.lock().unwrap(), Default::default());
        self
    }
}
/*
pub struct Delta<TDelta>(pub TDelta);

impl<TDelta> Delta<TDelta> {
    pub fn transfer<TValue>(self,a:&AState<TValue,TDelta>,b:&AState<TValue,TDelta>) 
        where 
        TDelta:Default+AddAssign<TDelta>+Neg<Output = TDelta>+Copy,
        TValue:AddAssign<TDelta>
    {
        let v_neg=-self.0;
        a.add_delta(self.0);
        b.add_delta(v_neg);
    }
} */