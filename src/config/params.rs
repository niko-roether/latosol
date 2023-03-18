use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// Start the latosol server.
pub struct Parameters {
	/// the port to listen to (default: 6969)
	#[argh(option, default = "6969")]
	pub port: u16,

	/// the maximum number of connections in the database connection pool (default: 100)
	#[argh(option, default = "100")]
	pub db_max_conns: u32
}

pub fn read() -> Parameters {
	argh::from_env()
}
