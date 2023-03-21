use mongodb::bson::Bson;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DbResultError {
	NoInsertedObjectId { actual: Bson }
}

impl fmt::Display for DbResultError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::NoInsertedObjectId { actual } => write!(
				f,
				"Expected an object id in response to insert operation; got {actual:?} instead"
			)
		}
	}
}

impl Error for DbResultError {}
