use std::{env, error::Error, fmt, fs::read_dir, iter};

use anyhow::{Context, Result};
use tokio::fs::File;
use tokio_rustls::rustls::{Certificate, PrivateKey};

use crate::server::TlsParams;

#[derive(Debug)]
pub struct DuplicatePrivateKeyError;

impl fmt::Display for DuplicatePrivateKeyError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "More than one private key was provided")
	}
}

impl Error for DuplicatePrivateKeyError {}

#[derive(Debug)]
pub struct NoPrivateKeyError;

impl fmt::Display for NoPrivateKeyError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "No private key was provided")
	}
}

impl Error for NoPrivateKeyError {}

#[derive(Debug)]
struct TlsParamsBuilder {
	certificates: Vec<Certificate>,
	private_key: Option<PrivateKey>
}

impl TlsParamsBuilder {
	fn new() -> Self {
		Self {
			certificates: vec![],
			private_key: None
		}
	}

	fn add_item(&mut self, item: rustls_pemfile::Item) -> Result<()> {
		use rustls_pemfile::Item::*;
		match item {
			X509Certificate(cert) => {
				log::debug!("Certificate loaded");
				self.certificates.push(Certificate(cert))
			}
			RSAKey(key) | PKCS8Key(key) => {
				match self.private_key {
					None => self.private_key = Some(PrivateKey(key)),
					Some(_) => return Err(DuplicatePrivateKeyError.into())
				}
				log::debug!("Private key loaded");
			}
			ECKey(_) => log::warn!(
				"A Sec1-encoded private key was provided; these are not supported and will be \
				 ignored."
			),
			_ => ()
		}
		Ok(())
	}

	async fn add_from_file(&mut self, file: File) -> Result<()> {
		let mut reader = std::io::BufReader::new(file.into_std().await);

		for item in iter::from_fn(|| rustls_pemfile::read_one(&mut reader).transpose()) {
			self.add_item(item?)?;
		}

		Ok(())
	}

	fn complete(self) -> Result<TlsParams> {
		Ok(TlsParams {
			certificates: self.certificates,
			private_key: self.private_key.ok_or(NoPrivateKeyError)?
		})
	}
}

pub async fn read_params() -> Result<TlsParams> {
	let conf_dir = env::var("LATOSOL_TLS_CONF_DIR").context("$LATOSOL_TLS_CONF_DIR is not set")?;
	let entries = read_dir(conf_dir.clone()).context(format!(
		"Failed to open {conf_dir}; make sure $LATOSOL_TLS_CONF_DIR is set correctly"
	))?;

	let mut tls_params_builder = TlsParamsBuilder::new();

	for entry in entries {
		let entry = entry?;
		let path = entry.path();
		let path_str = path.file_name().unwrap().to_string_lossy().into_owned();
		if path.is_dir() {
			log::warn!(
				"Found subdirectory '{path_str}' in TLS config directory. Subdirectories are not \
				 supported here and will be ignored.",
			);
			continue;
		}
		let Ok(file) = File::open(path).await else {
            log::error!("Failed to open '{path_str}', skipping...");
            continue;
        };
		log::debug!("Scanning '{path_str}' for certficates or private key...");
		tls_params_builder
			.add_from_file(file)
			.await
			.context("Error while reading '{path_str}'")?;
	}

	tls_params_builder.complete()
}
