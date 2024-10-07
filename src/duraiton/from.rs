use super::AbsoluteDuration;
use super::consts::*;

impl AbsoluteDuration {
    /// Created a new [AbsoluteDuration] from Nanoseconds
    /// 
    /// ```no_run
    /// use kale_duration::AbsoluteDuration;
    /// 
    /// let duration = AbsoluteDuration::from_nanos(22);
    /// ```
    pub fn from_nanos(nanoseconds: u128) -> Self {
        AbsoluteDuration {
            nanos: nanoseconds
        }
    }

    /// Create a new [AbsoluteDuration] from Microseconds
    /// 
    /// ```no_run
    /// use kale_duration::AbsoluteDuration;
    /// 
    /// let duration = AbsoluteDuration::from_micros(12);
    /// ```
    pub fn from_micros(microseconds: u128) -> Self {
        let nanos = microseconds * NANOS_IN_A_MICROSECOND;

        AbsoluteDuration { nanos }
    }
    
    /// Create a new [AbsoluteDuration] from Milliseconds
    /// 
    /// ```no_run
    /// use kale_duration::AbsoluteDuration;
    /// 
    /// let duration = AbsoluteDuration::from_micros(88);
    /// ```
    pub fn from_millis(milliseconds: u128) -> Self {
        let nanos = milliseconds * NANOS_IN_A_MILLISECOND;
        
        AbsoluteDuration { nanos }
    }

    /// Create a new [AbsoluteDuration] from Seconds
    /// 
    /// ```no_run
    /// use kale_duration::AbsoluteDuration;
    /// 
    /// let duration = AbsoluteDuration::from_seconds(8);
    /// ```
    pub fn from_seconds(seconds: u128) -> Self {
        let nanos = seconds as u128 * NANOS_IN_A_SECOND;
        
        AbsoluteDuration { nanos }
    }

    /// Create a new [AbsoluteDuration] from Minutes
    /// 
    /// ```no_run
    /// use kale_duration::AbsoluteDuration;
    /// 
    /// let duration = AbsoluteDuration::from_minutes(2);
    /// ```
    pub fn from_minutes(minutes: u64) -> Self {
        let nanos = minutes as u128 * NANOS_IN_A_MINUTE;

        AbsoluteDuration { nanos }
    }

    /// Create a new [AbsoluteDuration] from Hours
    /// 
    /// ```no_run
    /// use kale_duration::AbsoluteDuration;
    /// 
    /// let duration = AbsoluteDuration::from_hours(55);
    /// ```
    pub fn from_hours(hours: u64) -> Self {
        let nanos = hours as u128 * NANOS_IN_AN_HOUR;

        AbsoluteDuration { nanos }
    }

    /// Create a new [AbsoluteDuration] from Days
    /// 
    /// ```no_run
    /// use kale_duration::AbsoluteDuration;
    /// 
    /// let duration = AbsoluteDuration::from_days(3);
    /// ```
    pub fn from_days(days: u64) -> Self {
        let nanos = days as u128 * NANOS_IN_A_DAY;

        AbsoluteDuration { nanos }
    }

    /// Create a new [AbsoluteDuration] from Weeks
    /// 
    /// ```no_run
    /// use kale_duration::AbsoluteDuration;
    /// 
    /// let duration = AbsoluteDuration::from_weeks(59);
    /// ```
    pub fn from_weeks(weeks: u32) -> Self {
        let nanos = weeks as u128 * NANOS_IN_A_WEEK;

        AbsoluteDuration { nanos }
    }
}


#[cfg(test)]
mod tests {
    use crate::AbsoluteDuration;

    #[test]
    fn test_from_nanos() {
        let test_nanos = 32;

        let testee = AbsoluteDuration::from_nanos(test_nanos);

        assert_eq!(
            testee.nanos,
            32
        )
    }

    #[test]
    fn test_from_micros() {
        let test_micros = 42;

        let testee = AbsoluteDuration::from_micros(test_micros);

        assert_eq!(
            testee.nanos,
            42_000
        )
    }

    #[test]
    fn test_from_millis() {
        let test_millis = 52;

        let testee = AbsoluteDuration::from_millis(test_millis);

        assert_eq!(
            testee.nanos,
            52_000_000
        )
    }

    #[test]
    fn test_from_secs() {
        let test_secs = 4;

        let testee = AbsoluteDuration::from_seconds(test_secs);

        assert_eq!(
            testee.nanos,
            4_000_000_000
        )
    }

    #[test]
    fn test_from_minutes() {
        let test_mins = 2;

        let testee = AbsoluteDuration::from_minutes(test_mins);

        assert_eq!(
            testee.nanos,
            120_000_000_000
        )
    }

    #[test]
    fn test_from_hours() {
        let test_hours = 3;

        let testee = AbsoluteDuration::from_hours(test_hours);

        assert_eq!(
            testee.nanos,
            10_800_000_000_000
        )
    }

    #[test]
    fn test_from_days() {
        let test_hours = 5;

        let testee = AbsoluteDuration::from_days(test_hours);

        assert_eq!(
            testee.nanos,
            432_000_000_000_000
        )
    }

    #[test]
    fn test_from_weeks() {
        let test_hours = 1;

        let testee = AbsoluteDuration::from_weeks(test_hours);

        assert_eq!(
            testee.nanos,
            604_800_000_000_000
        )
    }
}