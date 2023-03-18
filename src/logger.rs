use env_logger::Env;

pub fn init() {
	let env = Env::default()
		.filter("LATOSOL_LOG")
		.write_style("LATOSOL_LOG_STYLE")
		.default_filter_or("info");
	env_logger::init_from_env(env);
}
