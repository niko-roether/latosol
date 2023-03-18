#![feature(async_closure)]

use std::{
	fs::File,
	io::{BufReader, Write}
};

use env_logger::Env;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use server::{Server, TlsParams};

mod server;

mod errors;

fn get_test_certs() -> Vec<Certificate> {
	let f = File::open("./test_crt/test.crt").unwrap();
	let mut reader = BufReader::new(f);
	certs(&mut reader)
		.unwrap()
		.into_iter()
		.map(Certificate)
		.collect()
}

fn get_test_key() -> PrivateKey {
	let f = File::open("./test_crt/test.key").unwrap();
	let mut reader = BufReader::new(f);
	let keys = pkcs8_private_keys(&mut reader).unwrap();
	PrivateKey(keys.into_iter().next().unwrap())
}

fn init_logger() {
	let env = Env::default().default_filter_or("info");
	env_logger::init_from_env(env);
}

#[tokio::main]
async fn main() {
	init_logger();

	let server = Server::bind(
		6969,
		TlsParams {
			certificates: get_test_certs(),
			private_key: get_test_key()
		}
	)
	.unwrap();

	server.listen(async move |mut conn| {
		{
			let mut writer = conn.writer();
			writer.write_all(&[69, 69, 69, 69, 0]).unwrap();
		}
		conn.send().unwrap();
	})
}
