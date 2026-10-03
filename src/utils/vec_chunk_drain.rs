

// pub struct 

pub struct DrainGuard<'a,T>{
	v:&'a mut Vec<T>
}

pub struct Drain<'a,T>{
	slice:&'a mut [T]
}