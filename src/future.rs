//! An async version of the [retry](crate::sync::retry) and
//! [retry_if](crate::sync::retry_if) function along with
//! [RetryFuture](crate::future::RetryFuture) used to implement them.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

use crate::Backoff;

/// Retries the provided function if it returns an error whenever the backoff
/// allows. The first call resulting in success will have it's value returned
/// to the caller.
///
/// # Example
/// ```
/// # use tryagain::*;
/// # async {
/// // In the real world this would do some computation that returns a result.
/// async fn returns_err() -> Result<(), ()> {
///     Err(())
/// }
///
/// // In this example we never get a value, we just spin forever.
/// let value = tryagain::future::retry(ExponentialBackoff::default(), || returns_err()).await;
/// # };
/// ```
pub fn retry<B, F, T, E, Fut>(
    backoff: B,
    func: F,
) -> RetryFuture<F, Fut, impl Fn(&E, u32) -> bool, B>
where
    B: Backoff,
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    retry_if(backoff, func, |_, _| true)
}

/// Calls the provided function and if an error is returned it is passed to
/// the predicate to determine if the function should be retried when the
/// backoff function allows.
///
/// # Example
/// ```
/// # use tryagain::*;
/// # async {
/// enum Error {
///     Recoverable,
///     Fatal,
/// }
///
/// async fn returns_fatal_error() -> Result<(), Error> {
///     Err(Error::Fatal)
/// }
///
/// // Returns a Result of Error::Fatal
/// let result = tryagain::future::retry_if(
///     ExponentialBackoff::default(),
///     || returns_fatal_error(),
///     |error, _iterations| match error {
///         Error::Fatal => false, // This error isn't recoverable.
///         _ => true,
///     },
/// ).await;
/// # };
/// ```
pub fn retry_if<B, F, P, T, E, Fut>(backoff: B, func: F, predicate: P) -> RetryFuture<F, Fut, P, B>
where
    B: Backoff,
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    P: Fn(&E, u32) -> bool,
{
    let future = func();

    RetryFuture {
        factory: func,
        future,
        predicate,
        backoff,
        paused_until: None,
        iterations: 0,
    }
}

#[pin_project::pin_project]
/// A future that will retry an operation.
pub struct RetryFuture<F, Fut, P, B> {
    factory: F,
    #[pin]
    future: Fut,
    predicate: P,
    backoff: B,
    paused_until: Option<Instant>,
    iterations: u32,
}

impl<T, E, F, Fut, P, B> Future for RetryFuture<F, Fut, P, B>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    P: Fn(&E, u32) -> bool,
    B: Backoff,
{
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        if let Some(paused_until) = this.paused_until {
            if Instant::now() < *paused_until {
                return Poll::Pending;
            }

            *this.paused_until = None;
        }

        let result = match this.future.as_mut().poll(cx) {
            Poll::Ready(res) => res,
            Poll::Pending => return Poll::Pending,
        };

        match result {
            Ok(value) => return Poll::Ready(Ok(value)),
            Err(e) => {
                *this.iterations += 1;
                let can_continue = (this.predicate)(&e, *this.iterations);

                if !can_continue {
                    return Poll::Ready(Err(e));
                }

                let new_future = (this.factory)();
                this.future.set(new_future);

                let duration = this.backoff.backoff_period(*this.iterations);
                let waker = cx.waker().clone();

                *this.paused_until = Some(Instant::now() + duration);

                // This is a hack to call the waker, I don't have a better way
                // to do this other than looping, which would block.
                #[cfg(feature = "runtime-tokio")]
                tokio::spawn(async move {
                    tokio::time::sleep(duration).await;
                    waker.wake();
                });

                #[cfg(feature = "runtime-async-std")]
                async_std::task::spawn(async move {
                    async_std::task::sleep(duration).await;
                    waker.wake();
                });

                Poll::Pending
            }
        }
    }
}
