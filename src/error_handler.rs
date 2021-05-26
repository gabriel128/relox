use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, where_it_was: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, where_it_was, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}

pub fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

pub fn set_error(val: bool) {
    HAD_ERROR.store(val, Ordering::Relaxed);
}
