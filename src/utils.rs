#[macro_export]
macro_rules! return_on_err {
	($err:expr, $val:expr) => {
		match $err {
			Ok(val) => val,
			Err(err) => {
				::log::error!("Fatal Error: {err:?}");
				::log::error!("Exiting...");
				return $val;
			}
		}
	};
	($err:expr) => {
		return_on_err!($err, ())
	};
}
