use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::crypt;

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	EntityNotFound { entity: &'static str, id: i64},
	Store(store::Error),
	Crypt(crypt::Error),
	Sqlx(#[serde_as(as = "DisplayFromStr")]sqlx::Error),
}

impl From<crypt::Error> for Error {
	fn from(val: crypt::Error) -> Self {
		Self::Crypt(val)
	}
}

impl From<store::Error> for Error {
	fn from(val: store::Error) -> Self {
		Self::Store(val)
	}
}

impl From<sqlx::Error> for Error {
	fn from(val: sqlx::Error) -> Self {
		Self::Sqlx(val)
	}
}


// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate