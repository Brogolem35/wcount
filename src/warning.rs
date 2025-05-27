use std::sync::atomic::{AtomicBool, Ordering};

static WARNING_PRINTED: AtomicBool = AtomicBool::new(false);

#[macro_export]
macro_rules! wprintln {
	($($tt:tt)*) => {
                $crate::warning::set_warning(true);
                eprintln!($($tt)*);
        };
}

#[inline]
pub fn set_warning(b: bool) {
	WARNING_PRINTED.store(b, Ordering::SeqCst);
}

#[inline]
pub fn warning_printed() -> bool {
	WARNING_PRINTED.load(Ordering::SeqCst)
}
