use bma_ts::Timestamp;
use rtc_interval::RtcInterval;
use std::time::Duration;

// ── sync ──────────────────────────────────────────────────────────────────────

/// Sub-second tick must fire promptly and return a timestamp close to now.
#[test]
fn sub_second_tick_is_recent() {
    let mut interval = RtcInterval::new(Duration::from_millis(100));
    let t = interval.tick();
    let skew = t.abs_diff(Timestamp::now());
    assert!(skew < Duration::from_secs(1), "tick skew too large: {skew:?}");
}

/// Sub-second ticks must be monotonically non-decreasing.
#[test]
fn sub_second_ticks_are_monotonic() {
    let mut interval = RtcInterval::new(Duration::from_millis(50));
    let t1 = interval.tick();
    let t2 = interval.tick();
    let t3 = interval.tick();
    assert!(t2.as_secs_f64() >= t1.as_secs_f64());
    assert!(t3.as_secs_f64() >= t2.as_secs_f64());
}

/// For intervals >= 1s the library returns `Timestamp::from_secs`, so the
/// sub-second part must be exactly zero.
#[test]
fn second_interval_tick_has_no_sub_second_component() {
    let mut interval = RtcInterval::new(Duration::from_secs(1));
    let t = interval.tick();
    assert_eq!(
        t.as_nanos() % 1_000_000_000,
        0,
        "tick for >= 1s interval must be a whole-second timestamp"
    );
}

/// 2-second interval must fire on an even unix second.
#[test]
fn two_second_interval_fires_on_even_second() {
    let mut interval = RtcInterval::new(Duration::from_secs(2));
    let t = interval.tick();
    assert_eq!(
        t.as_secs() % 2,
        0,
        "2s interval fired on odd second: {}",
        t.as_secs()
    );
}

/// Consecutive ticks of a 1s interval must return strictly increasing seconds
/// — the dedup guard must prevent the same second being delivered twice.
#[test]
fn consecutive_1s_ticks_never_repeat_the_same_second() {
    let mut interval = RtcInterval::new(Duration::from_secs(1));
    let t1 = interval.tick();
    let t2 = interval.tick();
    assert!(
        t2.as_secs() > t1.as_secs(),
        "second tick must be later: t1={} t2={}",
        t1.as_secs(),
        t2.as_secs()
    );
}

// ── async ─────────────────────────────────────────────────────────────────────

#[cfg(feature = "async")]
mod async_tests {
    use bma_ts::Timestamp;
    use rtc_interval::AsyncRtcInterval;
    use std::time::Duration;

    fn rt() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
    }

    #[test]
    fn async_sub_second_tick_is_recent() {
        rt().block_on(async {
            let mut interval = AsyncRtcInterval::new(Duration::from_millis(100));
            let t = interval.tick().await;
            let skew = t.abs_diff(Timestamp::now());
            assert!(skew < Duration::from_secs(1), "async tick skew too large: {skew:?}");
        });
    }

    #[test]
    fn async_sub_second_ticks_are_monotonic() {
        rt().block_on(async {
            let mut interval = AsyncRtcInterval::new(Duration::from_millis(50));
            let t1 = interval.tick().await;
            let t2 = interval.tick().await;
            let t3 = interval.tick().await;
            assert!(t2.as_secs_f64() >= t1.as_secs_f64());
            assert!(t3.as_secs_f64() >= t2.as_secs_f64());
        });
    }

    #[test]
    fn async_second_interval_tick_has_no_sub_second_component() {
        rt().block_on(async {
            let mut interval = AsyncRtcInterval::new(Duration::from_secs(1));
            let t = interval.tick().await;
            assert_eq!(
                t.as_nanos() % 1_000_000_000,
                0,
                "async tick for >= 1s interval must be a whole-second timestamp"
            );
        });
    }

    #[test]
    fn async_two_second_interval_fires_on_even_second() {
        rt().block_on(async {
            let mut interval = AsyncRtcInterval::new(Duration::from_secs(2));
            let t = interval.tick().await;
            assert_eq!(
                t.as_secs() % 2,
                0,
                "async 2s interval fired on odd second: {}",
                t.as_secs()
            );
        });
    }

    #[test]
    fn async_consecutive_1s_ticks_never_repeat_the_same_second() {
        rt().block_on(async {
            let mut interval = AsyncRtcInterval::new(Duration::from_secs(1));
            let t1 = interval.tick().await;
            let t2 = interval.tick().await;
            assert!(
                t2.as_secs() > t1.as_secs(),
                "async second tick must be later: t1={} t2={}",
                t1.as_secs(),
                t2.as_secs()
            );
        });
    }
}
