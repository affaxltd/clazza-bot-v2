use std::time::{Duration, SystemTime};

use crate::async_lock::{AsyncLock, IntoLock};

#[derive(Clone)]
pub struct Timer {
    time: AsyncLock<SystemTime>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: SystemTime::now().into_lock(),
        }
    }

    pub fn new_time(time: SystemTime) -> Self {
        Self {
            time: time.into_lock(),
        }
    }

    pub async fn since(&self, time: &SystemTime) -> Duration {
        let then = self.time.read().await;

        time.duration_since(*then)
            .unwrap_or_else(|_| Duration::from_nanos(0))
    }

    pub async fn since_now(&self) -> Duration {
        self.since(&SystemTime::now()).await
    }

    pub async fn update_time(&self, time: SystemTime) {
        *self.time.write().await = time;
    }

    pub async fn update_time_now(&self) {
        self.update_time(SystemTime::now()).await;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Cooldown {
    timer: Timer,
}

impl Cooldown {
    pub fn new() -> Self {
        Self {
            timer: Timer::new_time(SystemTime::UNIX_EPOCH),
        }
    }

    pub async fn is_done(&self, time: u128) -> bool {
        self.timer.since_now().await.as_millis() >= time
    }

    pub async fn update_time(&self) {
        self.timer.update_time_now().await;
    }

    pub async fn cooldown_passed(&self, time: u128) -> bool {
        if self.is_done(time).await {
            self.update_time().await;
            true
        } else {
            false
        }
    }
}

impl Default for Cooldown {
    fn default() -> Self {
        Self::new()
    }
}
