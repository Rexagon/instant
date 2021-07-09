pub type Instant = std::time::Instant;

/// The current time, in milliseconds.
#[cfg(feature = "now")]
pub fn now() -> f64 {
    use std::time::SystemTime;

    (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH))
        .expect("System clock was before 1970.")
        .as_secs_f64()
        * 1000.0
}
