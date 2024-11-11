#[cfg(feature = "instant")]
pub use instant::Instant;
#[cfg(not(feature = "instant"))]
pub use std::time::Instant;
