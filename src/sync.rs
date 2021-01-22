use crate::Backoff;

pub fn retry<B, F, T, E>(backoff: B, func: F) -> T
where
    B: Backoff,
    F: Fn() -> Result<T, E>,
{
    match retry_if(backoff, func, |_| true) {
        Ok(value) => value,
        Err(_) => unreachable!(),
    }
}

pub fn retry_if<B, F, P, T, E>(mut backoff: B, func: F, predicate: P) -> Result<T, E>
where
    B: Backoff,
    F: Fn() -> Result<T, E>,
    P: Fn(&E) -> bool,
{
    let mut iterations = 0;

    loop {
        match func() {
            Ok(value) => return Ok(value),
            Err(e) => {
                if !predicate(&e) {
                    return Err(e);
                }

                loop {
                    if backoff.should_try_again(iterations) {
                        break;
                    }
                    std::thread::yield_now();
                }
            }
        }

        iterations += 1;
    }
}
