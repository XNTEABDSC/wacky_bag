
/// A value with expected meaning
/// see [`Has`] for more information
pub trait HasMarker{
	type Item;
}

/// Represents that you can get value with expected [`HasMarker`].
/// You can also use `Marker` as parameters and use [`Has`] to act like a function.
pub trait Has<Marker:HasMarker>{
	fn get_has(self,marker:Marker)->Marker::Item;
}