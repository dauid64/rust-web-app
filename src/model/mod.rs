// region:    --- Modules

mod base;
mod error;
mod store;
pub mod task;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		// FIXME - TBC
		Ok(ModelManager { db })
	}

	pub(in crate::model) fn db(&self) -> &Db { // acessivel somente para pastas filhas a model
		&self.db
	}
}