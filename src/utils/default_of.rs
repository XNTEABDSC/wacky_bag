//! [default]


/// `default::<T>()` instead of [`Default::default()`] or `<T as Default>::default()`
pub fn default<T:Default>()->T{T::default()}