#[macro_export]
macro_rules! fatal_error {
	($err:expr) => {
		match $err {
			Ok(val) => val,
			Err(err) => {
				::log::error!("Fatal Error: {err:?}");
				::log::error!("Exiting...");
				::std::process::exit(1);
			}
		}
	};
}
