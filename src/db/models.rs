use mongodb::bson;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
#[serde(renameAll = "camelCase")]
pub struct Asset {
	id: bson::Uuid,
	mime_type: String,
	data: Vec<u8>
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DeviceType {
	Unknown = 0,
	Mobile = 1,
	Desktop = 2,
	Cli = 3,
	Web = 4
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(renameAll = "camelCase")]
pub struct Device {
	id: bson::Uuid,
	name: String,
	#[serde(rename = "type")]
	device_type: DeviceType,
	user: bson::Uuid,
	last_activity: bson::DateTime,
	signature_public_key: bson::Binary,
	encryption_public_key: bson::Binary
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(renameAll = "camelCase")]
pub struct MessageRecipient {
	device: bson::Uuid,
	key: bson::Binary
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(renameAll = "camelCase")]
pub struct Message {
	id: bson::Uuid,
	from_user: bson::Uuid,
	to_user: bson::Uuid,
	recipients: Vec<MessageRecipient>,
	iv: bson::Binary,
	sent_at: bson::DateTime,
	content_type: String,
	body: bson::Binary
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(renameAll = "camelCase")]
pub struct User {
	id: bson::Uuid,
	username: String,
	display: String,
	password_hash: String,
	status: String,
	profile_picture: bson::Uuid,
	created_at: bson::DateTime,
	updated_at: bson::DateTime
}
