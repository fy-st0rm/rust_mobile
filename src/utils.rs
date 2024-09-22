
#[macro_export]
macro_rules! static_assert {
	($cond: expr, $msg: expr) => {
		const _: () = {
			assert!($cond, "{}", $msg);
		};
	}
}
