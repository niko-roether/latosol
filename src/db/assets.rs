use anyhow::{Context, Result};
use mongodb::bson;

use crate::{db::results::inserted_id, models::Asset};

use super::{Doc, Id};

pub struct AssetCollection {
	collection: mongodb::Collection<Asset>
}

impl AssetCollection {
	const COLLECTION: &str = "assets";

	pub fn new(db: &mongodb::Database) -> Self {
		let collection = db.collection(Self::COLLECTION);
		Self { collection }
	}

	pub async fn save_asset(&self, asset: &Asset) -> Result<Id> {
		let res = self
			.collection
			.insert_one(asset, None)
			.await
			.context("Failed to save asset to database")?;

		Ok(inserted_id(res)?.into())
	}
}
