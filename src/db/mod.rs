use anyhow::{Context, Result};
use mongodb::{bson, options::ClientOptions};

mod assets;

mod results;

mod error;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id([u8; 12]);

impl Id {
	pub fn from_bytes(bytes: [u8; 12]) -> Self {
		Self(bytes)
	}

	pub fn bytes(&self) -> [u8; 12] {
		self.0
	}
}

impl From<bson::oid::ObjectId> for Id {
	fn from(value: bson::oid::ObjectId) -> Self {
		Self::from_bytes(value.bytes())
	}
}

impl From<Id> for bson::oid::ObjectId {
	fn from(value: Id) -> Self {
		bson::oid::ObjectId::from_bytes(value.bytes())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Doc<T> {
	id: Id,
	value: T
}

impl<T> Doc<T> {
	fn new(id: Id, value: T) -> Self {
		Self { id, value }
	}

	pub fn id(&self) -> Id {
		self.id
	}

	pub fn value(&self) -> &'_ T {
		&self.value
	}

	pub fn value_mut(&mut self) -> &'_ mut T {
		&mut self.value
	}

	pub fn into_value(self) -> T {
		self.value
	}
}
