#[cfg(feature = "async")]
pub use async_interval::AsyncRtcInterval;

use std::time::Duration;

use bma_ts::Timestamp;

pub struct RtcInterval {
    interval: rtsc::time::Interval,
    d: Duration,
    tick_prev: u64,
}

impl RtcInterval {
    pub fn new(d: Duration) -> Self {
        let mut interval = rtsc::time::interval(if d >= Duration::from_secs(1) {
            Duration::from_millis(200)
        } else {
            d
        });
        interval = interval.set_missing_tick_behavior(rtsc::time::MissedTickBehavior::Skip);
        Self {
            interval,
            d,
            tick_prev: 0,
        }
    }
    pub fn tick(&mut self) -> Timestamp {
        if self.d >= Duration::from_secs(1) {
            loop {
                self.interval.tick();
                let t_secs = Timestamp::now().as_secs();
                if t_secs.is_multiple_of(self.d.as_secs()) && t_secs != self.tick_prev {
                    self.tick_prev = t_secs;
                    break Timestamp::from_secs(t_secs);
                }
            }
        } else {
            self.interval.tick();
            Timestamp::now()
        }
    }
}

#[cfg(feature = "async")]
mod async_interval {
    use std::time::Duration;

    use bma_ts::Timestamp;

    pub struct AsyncRtcInterval {
        interval: tokio::time::Interval,
        d: Duration,
        tick_prev: u64,
    }

    impl AsyncRtcInterval {
        pub fn new(d: Duration) -> Self {
            let mut interval = tokio::time::interval(if d >= Duration::from_secs(1) {
                Duration::from_millis(200)
            } else {
                d
            });
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            Self {
                interval,
                d,
                tick_prev: 0,
            }
        }
        pub async fn tick(&mut self) -> Timestamp {
            if self.d >= Duration::from_secs(1) {
                loop {
                    self.interval.tick().await;
                    let t_secs = Timestamp::now().as_secs();
                    if t_secs.is_multiple_of(self.d.as_secs()) && t_secs != self.tick_prev {
                        self.tick_prev = t_secs;
                        break Timestamp::from_secs(t_secs);
                    }
                }
            } else {
                self.interval.tick().await;
                Timestamp::now()
            }
        }
    }
}
