#![cfg(feature = "std")]

use futures::SinkExt;
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
    fn sink() {
        let i = Instant::now();
        let lim = Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(10u32))));
        let mut sink = Vec::new().ratelimit_sink(&lim);

        for _ in 0..10 {
            wait!(sink.send(())).unwrap();
        }
        assert!(
            i.elapsed() <= Duration::from_millis(100),
            "elapsed: {:?}",
            i.elapsed()
        );

        wait!(sink.send(())).unwrap();
        assert!(
            i.elapsed() > Duration::from_millis(100),
            "elapsed: {:?}",
            i.elapsed()
        );
        assert!(
            i.elapsed() <= Duration::from_millis(200),
            "elapsed: {:?}",
            i.elapsed()
        );

        wait!(sink.send(())).unwrap();
        assert!(i.elapsed() > Duration::from_millis(200), "elapsed: {:?}", i.elapsed());
        assert!(i.elapsed() <= Duration::from_millis(300), "elapsed: {:?}", i.elapsed());

        let result = sink.into_inner();
        assert_eq!(result.len(), 12);
        assert!(result.into_iter().all(|elt| elt == ()));
    }
}
