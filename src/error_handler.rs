use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn runtime_error(line: usize, message: &str) {
    eprintln!("[line {}] RuntimError: {}", line, message);
    set_runtime_error(false);
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

pub fn had_runtime_error() -> bool {
    HAD_RUNTIME_ERROR.load(Ordering::Relaxed)
}

pub fn set_runtime_error(val: bool) {
    HAD_RUNTIME_ERROR.store(val, Ordering::Relaxed);
}
