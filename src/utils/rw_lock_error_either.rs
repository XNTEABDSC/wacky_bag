//! [`RwLockErrorEither`]

use std::{error::Error, fmt::Display, sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard}};

use either::Either;

/// 2 error from [`RwLock::read`][std::sync::RwLock::read] and [`RwLock::write`][std::sync::RwLock::write]
#[derive(Debug)]
pub struct RwLockErrorEither<'a,T>(pub Either<
	PoisonError<RwLockReadGuard<'a, T>>,
	PoisonError<RwLockWriteGuard<'a, T>>
>);

impl<'a, T: std::fmt::Debug> Error for RwLockErrorEither<'a, T> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.0.source()
	}

	#[allow(deprecated)]
	fn description(&self) -> &str {
		// "description() is deprecated; use Display"
		self.0.description()
	}

	#[allow(deprecated)]
	fn cause(&self) -> Option<&dyn Error> {
		self.0.cause()
	}

	// fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {
	// 	self.0.provide(request);
	// }
}

impl<'a,T> Display for RwLockErrorEither<'a,T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl<'a,T> From<PoisonError<RwLockReadGuard<'a, T>>> for RwLockErrorEither<'a,T> {
	fn from(value: PoisonError<RwLockReadGuard<'a, T>>) -> Self {
		Self(Either::Left(value))
	}
}
impl<'a,T> From<PoisonError<RwLockWriteGuard<'a, T>>> for RwLockErrorEither<'a,T> {
	fn from(value: PoisonError<RwLockWriteGuard<'a, T>>) -> Self {
		Self(Either::Right(value))
	}
}
