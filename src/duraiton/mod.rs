mod from;
mod consts;
mod r#as;
mod cmp;

#[cfg(feature = "serde")]
mod deserialize;

/// A Duration that is absolute (i.e. can only represent a positive duration and not go into the negative)
#[derive(Debug, Clone, Copy)]
pub struct AbsoluteDuration {
    nanos: u128
}

/// Converts an [AbsoluteDuration] to a [standard Duration](struct@std::time::Duration)
/// 
/// # Undefined behaivor
/// This might cause undefined behaivor because duration only supports duration from nanos as [u64] instead of [u128].
impl From<AbsoluteDuration> for std::time::Duration {
    fn from(value: AbsoluteDuration) -> Self {
        std::time::Duration::from_nanos(value.nanos as u64)
    }
}

impl From<std::time::Duration> for AbsoluteDuration {
    fn from(value: std::time::Duration) -> Self {
        Self { nanos: value.as_nanos() }
    }
}