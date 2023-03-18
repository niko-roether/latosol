use anyhow::{Context, Result};
use mongodb::options::ClientOptions;

#[derive(Debug)]
pub struct DatabaseLocation {
	pub uri: String,
	pub database: String
}

#[derive(Debug)]
pub struct Database {
	db: mongodb::Database
}

impl Database {
	pub async fn connect(
		DatabaseLocation { uri, database }: DatabaseLocation,
		max_conns: u32
	) -> Result<Self> {
		log::debug!("Attempting to connect to database...");

		let mut client_options = ClientOptions::parse(uri)
			.await
			.context("Malformed database URI")?;
		client_options.app_name = Some(String::from("latosol"));
		client_options.max_pool_size = Some(max_conns);

		let client = mongodb::Client::with_options(client_options)?;

		log::debug!("Database connection established.");

		let db = client.database(&database);

		Ok(Self { db })
	}
}
