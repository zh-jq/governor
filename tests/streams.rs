#![cfg(feature = "std")]

use futures::{stream, StreamExt};
use governor::{prelude::*, Quota, RateLimiter};
use instant::Instant;
use nonzero_ext::*;
use std::sync::Arc;
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use futures::executor::block_on;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[macro_use]
mod macros;

tests! {
    fn stream() {
        let i = Instant::now();
        let lim = Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(10u32))));
        let mut stream = stream::repeat(()).ratelimit_stream(&lim);

        for _ in 0..10 {
            wait!(stream.next());
        }
        let elapsed = i.elapsed();
        assert!(
            elapsed <= Duration::from_millis(100),
            "expected elapsed <= Duration::from_millis(100), elapsed: {:?}",
            elapsed
        );

        wait!(stream.next());

        let elapsed = i.elapsed();
        assert!(
            elapsed >= Duration::from_millis(100),
            "expected elapsed >= Duration::from_millis(100), elapsed: {:?}",
            elapsed
        );
        assert!(
            elapsed <= Duration::from_millis(200),
            "expected elapsed <= Duration::from_millis(200), elapsed: {:?}",
            elapsed
        );

        wait!(stream.next());

        let elapsed = i.elapsed();
        assert!(
            elapsed >= Duration::from_millis(200),
            "expected elapsed >= Duration::from_millis(200), elapsed: {:?}",
            elapsed
        );
        assert!(
            elapsed <= Duration::from_millis(300),
            "expected elapsed <= Duration::from_millis(300), elapsed: {:?}",
            elapsed
        );
    }
}
