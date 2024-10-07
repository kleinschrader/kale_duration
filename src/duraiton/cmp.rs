use super::AbsoluteDuration;

impl PartialEq for AbsoluteDuration {
    fn eq(&self, other: &Self) -> bool {
        self.nanos == other.nanos
    }
}

impl Eq for AbsoluteDuration {}

impl PartialOrd for AbsoluteDuration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.nanos.partial_cmp(&other.nanos)
    }
}

impl Ord for AbsoluteDuration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.nanos.cmp(&other.nanos)
    }
}