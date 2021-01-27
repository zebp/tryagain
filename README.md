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
let counter = Rc::new(RefCell::new(0));
let fails_four_times = || {
    let mut counter = counter.borrow_mut();
    *counter += 1;

    if *counter < 5 {
        Err(*counter)
    } else {
        Ok(())
    }
};

tryagain::retry(ImmediateBackoff, fails_four_times);
```
## Async example
```rust
let counter = Rc::new(RefCell::new(0));
let fails_four_times = || async {
    let mut counter = counter.borrow_mut();
    *counter += 1;

    if *counter < 5 {
        Err(*counter)
    } else {
        Ok(())
    }
};

tryagain::future::retry(ImmediateBackoff, fails_four_times).await;
```
