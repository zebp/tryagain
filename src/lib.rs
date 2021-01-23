#![forbid(unsafe_code)]

#[cfg(any(feature = "runtime-tokio", feature = "runtime-async-std"))]
pub mod future;

mod sync;
mod backoff;

pub use backoff::*;
pub use sync::*;