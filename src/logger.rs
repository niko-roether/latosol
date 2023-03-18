use env_logger::Env;

pub fn init() {
	let env = Env::default()
		.filter_or("LATOSOL_LOG", "info,latosol=debug")
		.write_style("LATOSOL_LOG_STYLE");
	env_logger::init_from_env(env);
}
