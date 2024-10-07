use super::AbsoluteDuration;
use super::consts::*;

impl AbsoluteDuration {
    /// Returns the whole [AbsoluteDuration] as Nanoseconds
    pub fn as_nanos(&self) -> u128 {
        self.nanos
    }

    /// Returns the whole [AbsoluteDuration] as Microseconds
    pub fn as_micros(&self) -> u128 {
        self.nanos / NANOS_IN_A_MICROSECOND
    }

    /// Returns the whole [AbsoluteDuration] as Milliseconds
    pub fn as_millis(&self) -> u128 {
        self.nanos / NANOS_IN_A_MILLISECOND
    }

    /// Returns the whole [AbsoluteDuration] as Seconds
    pub fn as_secs(&self) -> u128 {
        self.nanos / NANOS_IN_A_SECOND
    }

    /// Returns the whole [AbsoluteDuration] as Minutes
    pub fn as_mins(&self) -> u64 {
        (self.nanos / NANOS_IN_A_MINUTE) as u64
    }

    /// Returns the whole [AbsoluteDuration] as Hours
    pub fn as_hours(&self) -> u64 {
        (self.nanos / NANOS_IN_AN_HOUR) as u64
    }

    /// Returns the whole [AbsoluteDuration] as Days
    pub fn as_days(&self) -> u64 {
        (self.nanos / NANOS_IN_A_DAY) as u64
    }

    /// Returns the whole [AbsoluteDuration] as Weeks
    pub fn as_weeks(&self) -> u32 {
        (self.nanos / NANOS_IN_A_WEEK) as u32
    }
}
