use std::time::{Duration, Instant};

pub fn stopwatch<F, R>(callback: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let timer = Instant::now();

    let result = callback();

    (result, timer.elapsed())
}
