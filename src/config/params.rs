use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// Start the latosol server.
pub struct Parameters {
	/// the port to listen to
	#[argh(option, default = "6969")]
	pub port: u16
}

pub fn read() -> Parameters {
	argh::from_env()
}
