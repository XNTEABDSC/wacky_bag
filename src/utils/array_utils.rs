use std::{array, mem::ManuallyDrop, ptr};


pub fn array_from_2arr<A,B,C,Func,const N:usize>(a:[A;N],b:[B;N],func:Func)->[C;N]
    where Func:Fn(A,B)->C
{
    let a = ManuallyDrop::new(a);
    let b = ManuallyDrop::new(b);
    
    // 获取原始指针
    let a_ptr = a.as_ptr() as *const A;
    let b_ptr = b.as_ptr() as *const B;
    
    array::from_fn(|i|{
        func(unsafe {ptr::read(
        a_ptr.add(i)
        )},unsafe {
            ptr::read(b_ptr.add(i))})
    })
}

/*
fn array_add<T: std::ops::Add<Output = T>, const N: usize>(a: [T; N], b: [T; N]) -> [T; N] {
	let mut a_iter=a.into_iter();
	let mut b_iter=b.into_iter();
	array::from_fn(|_|{a_iter.next().unwrap()+b_iter.next().unwrap()})
} */