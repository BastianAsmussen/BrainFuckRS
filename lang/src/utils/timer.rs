use std::time::Duration;

pub fn time<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = std::time::Instant::now();
    let result = f();

    (result, start.elapsed())
}
