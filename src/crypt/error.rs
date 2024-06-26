use serde::Serialize;

use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	KeyFailHmac,
    PwdNotMatching,
	TokenInvalidFormat,
	TokenCannotDecodeIdent,
	TokenCannotDecodeExp,
	TokenSignatureNotMatching,
	TokenExpNotIso,
	TokenExpired,
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