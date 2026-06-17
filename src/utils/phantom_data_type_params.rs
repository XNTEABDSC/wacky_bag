
#[macro_export]
macro_rules! phantom_data_type_params{
	($lt:lifetime)=>{&$lt ()};
	($lt:lifetime, $($tt:tt),*)=>{
		(&$lt (), $crate::phantom_data_type_params!($($tt),*))
	};
	($ty:ty)=>{$ty};
	($ty:ty, $($tt:tt),*)=>{
		($ty, $crate::phantom_data_type_params!($($tt),*))
	};
	
}


#[macro_export]
macro_rules! ttttest{

	// ($($lt:lifetime)+ $($ty:ty)*)=>{f32};
	($ty:ty)=>{i32};
	($lt:lifetime)=>{i64};
}
#[cfg(test)]
mod test{
	use std::marker::PhantomData;

	fn test(){
		let a:PhantomData< phantom_data_type_params!('static,'static) >;
		let b:PhantomData< phantom_data_type_params!('static,'static, i32,i64) >;
	}
}