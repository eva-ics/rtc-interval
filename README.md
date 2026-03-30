<h2>
  Real-time clock synced intervals for Rust
  <a href="https://crates.io/crates/rtc-interval"><img alt="crates.io page" src="https://img.shields.io/crates/v/rtc-interval.svg"></img></a>
  <a href="https://docs.rs/rtc-interval"><img alt="docs.rs page" src="https://docs.rs/rtc-interval/badge.svg"></img></a>
  <a href="https://github.com/eva-ics/rtc-interval/actions/workflows/ci.yml">
    <img alt="GitHub Actions CI" src="https://github.com/eva-ics/rtc-interval/actions/workflows/ci.yml/badge.svg"></img>
  </a>
</h2>


Async and sync intervals that are synced to the real-time-clock.

That means e.g. a 10-second interval will fire at 00:00:00, 00:00:10, 00:00:20,
etc. instead of 10 seconds after the interval was started.

Note: for intervals shorter than 1 second does not sync to the real-time-clock,
working as a regular interval only.

## Examples

### Sync

```rust
use rtc_interval::RtcInterval;
use std::time::Duration;

let mut interval = RtcInterval::new(Duration::from_secs(10));
loop {
    let t = interval.tick(); // blocks until the next 10-second boundary
    println!("tick at {}", t.as_secs());
}
```

### Async

```rust
use rtc_interval::AsyncRtcInterval;
use std::time::Duration;

let mut interval = AsyncRtcInterval::new(Duration::from_secs(10));
loop {
    let t = interval.tick().await; // waits until the next 10-second boundary
    println!("tick at {}", t.as_secs());
}
```
