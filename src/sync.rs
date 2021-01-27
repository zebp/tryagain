use crate::Backoff;

/// Retries the provided function if it returns an error whenever the backoff
/// allows. The first call resulting in success will have it's value returned
/// to the caller.
///
/// # Example
/// ```
/// # use tryagain::*;
/// // In the real world this would do some computation that returns a result.
/// fn returns_err() -> Result<(), ()> {
/// #   return Ok(()); // Hack so our doc-tests pass
///     Err(())
/// }
///
/// // In this example we never get a value, we just spin forever.
/// let value = retry(ExponentialBackoff::default(), returns_err);
/// # assert_eq!(value, ());
/// ```
pub fn retry<B, F, T, E>(backoff: B, func: F) -> T
where
    B: Backoff,
    F: Fn() -> Result<T, E>,
{
    match retry_if(backoff, func, |_, _| true) {
        Ok(value) => value,
        Err(_) => unreachable!(),
    }
}

/// Calls the provided function and if an error is returned it is passed to
/// the predicate to determine if the function should be retried when the
/// backoff function allows.
///
/// # Example
/// ```
/// # use tryagain::*;
/// enum Error {
///     Recoverable,
///     Fatal,
/// }
///
/// fn returns_fatal_error() -> Result<(), Error> {
///     Err(Error::Fatal)
/// }
///
/// // Returns a Result of Error::Fatal
/// let result = tryagain::retry_if(
///     ExponentialBackoff::default(),
///     returns_fatal_error,
///     |error, _iterations| match error {
///         Error::Fatal => false, // This error isn't recoverable.
///         _ => true,
///     },
/// );
/// # result.expect_err("expected fatal error from result");
/// ```
pub fn retry_if<B, F, P, T, E>(mut backoff: B, func: F, predicate: P) -> Result<T, E>
where
    B: Backoff,
    F: Fn() -> Result<T, E>,
    P: Fn(&E, u32) -> bool,
{
    let mut iterations = 0;

    loop {
        match func() {
            Ok(value) => return Ok(value),
            Err(e) => {
                if !predicate(&e, iterations) {
                    return Err(e);
                }

                std::thread::sleep(backoff.backoff_period(iterations));
            }
        }

        iterations += 1;
    }
}
