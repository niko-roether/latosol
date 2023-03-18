use std::env;

use anyhow::{Context, Result};

use crate::db::DatabaseLocation;

fn read_mongodb_uri() -> Result<String> {
	env::var("LATOSOL_MONGODB_URI").context("$LATOSOL_MONGODB_URI is not set!")
}

fn read_database_name() -> Result<String> {
	env::var("LATOSOL_MONGODB_DATABASE").context("$LATOSOL_MONGODB_DATABASE is not set!")
}

pub fn read_location() -> Result<DatabaseLocation> {
	Ok(DatabaseLocation {
		uri: read_mongodb_uri()?,
		database: read_database_name()?
	})
}
