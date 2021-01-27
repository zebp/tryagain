//! # A crate for trying things again.
//! `tryagain` is a crate to try things again if they fail inspired by
//! [backoff](https://docs.rs/backoff) that offers an easier way to cancel
//! retry attemps and uses a non-blocking async implementation.
//! ## Sync example
//! ```
//! # use tryagain::*;
//! fn fails() -> Result<(), i32> {
//! #   return Ok(()); // So our doctests pass.
//!     Err(0)
//! }
//!
//! // Will never resolve into, will spin forever.
//! let value = tryagain::retry(ImmediateBackoff, fails);
//! ```
//! ## Async example
//! ```
//! # use tryagain::*;
//! # async {
//! async fn fails() -> Result<(), i32> {
//! #   return Ok(()); // So our doctests pass.
//!     Err(0)
//! }
//!
//! // Will never resolve into, will spin forever.
//! let value = tryagain::future::retry(ImmediateBackoff, fails).await;
//! # };
//! ```

#![forbid(unsafe_code)]

#[cfg(any(feature = "runtime-tokio", feature = "runtime-async-std"))]
pub mod future;

mod backoff;
mod sync;

pub use backoff::*;
pub use sync::*;
