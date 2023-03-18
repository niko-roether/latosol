use log::error;

macro_rules! define_param {
	($func_name:ident : $ty:ty = env($env_var_name:literal) | $default:expr) => {
		pub fn $func_name() -> $ty {
			if let Ok(str_val) = ::std::env::var($env_var_name) {
				if let Ok(value) = str_val.parse() {
					return value;
				}
				error!(
					"Invalid value for {}: should be of type {}. Falling back to default.",
					$env_var_name,
					stringify!($ty)
				);
			}
			$default
		}
	};
}

define_param!(port: u16 = env("LATOSOL_PORT") | 6969);
