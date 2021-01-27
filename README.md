# tryagain
[![Docs.rs][docs-badge]][docs-url]
[![Crates.io][crates-badge]][crates-url]
[![Unlicense][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/tryagain.svg
[crates-url]: https://crates.io/crates/tryagain
[license-badge]: https://img.shields.io/badge/license-Unlicense-blue.svg
[license-url]: https://github.com/vlakreeh/tryagain/blob/master/LICENSE
[docs-badge]: https://img.shields.io/badge/docs.rs-rustdoc-green
[docs-url]: https://docs.rs/tryagain/

`tryagain` is a crate to try things again if they fail inspired by
[backoff](https://docs.rs/backoff) that offers an easier way to cancel
retry attemps and uses a non-blocking async implementation.
`tryagain` works with both [tokio](https://crates.io/crates/tokio) and
[async-std](https://crates.io/crates/async-std) through the use of the
feature flags `runtime-tokio` and `runtime-async-std`.

## Sync example
```rust
fn fails() -> Result<(), i32> {
    Err(0)
}
// Will never resolve into, will spin forever.
let value = tryagain::retry(ImmediateBackoff, fails);
```
## Async example
```rust
async fn fails() -> Result<(), i32> {
    Err(0)
}
// Will never resolve into, will spin forever.
let value = tryagain::future::retry(ImmediateBackoff, fails).await;
```
