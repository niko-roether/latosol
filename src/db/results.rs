use anyhow::Result;
use mongodb::{
	bson::{self, Bson},
	results::InsertOneResult
};

use crate::db::error::DbResultError;

pub fn inserted_id(res: InsertOneResult) -> Result<bson::oid::ObjectId> {
	let Bson::ObjectId(oid) = res.inserted_id else {
        log::error!("Received something other than an inserted object id in response to InsertOne operation! This should never happen, and indicates an issue with the database.");
        return Err(DbResultError::NoInsertedObjectId { actual: res.inserted_id }.into());
    };
	Ok(oid)
}
