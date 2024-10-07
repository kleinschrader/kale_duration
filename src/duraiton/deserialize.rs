use serde::{de::{self, MapAccess, Visitor}, Deserialize, Deserializer};
use super::AbsoluteDuration;
use super::consts;

impl<'de> Deserialize<'de> for AbsoluteDuration {
    /// Deserialize this value from the given Serde deserializer.
    /// 
    /// You can provide the following fields:
    /// ```json
    /// {
    ///     nanoseconds: 12,
    ///     microseconds: 32,
    ///     millisseconds: 23,
    ///     seconds: 10,
    ///     hours: 2,
    ///     days: 5,
    ///     weeks: 1
    /// }
    /// ```
    /// 
    /// It will fail if you fail to provide any duration.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Nanos,
            Micros,
            Millis,
            Secs,
            Mins,
            Hours,
            Days,
            Weeks,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`nanoseconds`, `microseconds`, `milliseconds`, `seconds`, `minutes`, `hours`, `days` or `weeks`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "nanoseconds" => Ok(Field::Nanos),
                            "microseconds" => Ok(Field::Micros),
                            "milliseconds" => Ok(Field::Millis),
                            "seconds" => Ok(Field::Secs),
                            "minutes" => Ok(Field::Mins),
                            "hours" => Ok(Field::Hours),
                            "days" => Ok(Field::Days),
                            "weeks" => Ok(Field::Weeks),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct AbsoluteDurationVisitor;

        impl<'de> Visitor<'de> for AbsoluteDurationVisitor {
            type Value = AbsoluteDuration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct AbsoluteDuration")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut nanos = None;
                let mut micros = None;
                let mut millis = None;
                let mut secs = None;
                let mut mins = None;
                let mut hours = None;
                let mut days = None;
                let mut weeks = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Nanos => {
                            if nanos.is_some() {
                                return Err(de::Error::duplicate_field("nanoseconds"));
                            }

                            nanos = Some(map.next_value()?);
                        }
                        Field::Micros => {
                            if micros.is_some() {
                                return Err(de::Error::duplicate_field("microseconds"));
                            }

                            micros = Some(map.next_value()?);
                        },
                        Field::Millis => {
                            if millis.is_some() {
                                return Err(de::Error::duplicate_field("milliseconds"));
                            }

                            millis = Some(map.next_value()?);
                        },
                        Field::Secs => {
                            if secs.is_some() {
                                return Err(de::Error::duplicate_field("seconds"));
                            }

                            secs = Some(map.next_value()?);
                        },
                        Field::Mins => {
                            if mins.is_some() {
                                return Err(de::Error::duplicate_field("minutes"));
                            }

                            mins = Some(map.next_value()?);
                        },
                        Field::Hours => {
                            if hours.is_some() {
                                return Err(de::Error::duplicate_field("hours"));
                            }

                            hours = Some(map.next_value()?);
                        },
                        Field::Days => {
                            if days.is_some() {
                                return Err(de::Error::duplicate_field("days"));
                            }

                            days = Some(map.next_value()?);
                        },
                        Field::Weeks => {
                            if weeks.is_some() {
                                return Err(de::Error::duplicate_field("weeks"));
                            }

                            weeks = Some(map.next_value()?);
                        }
                    }
                }

                if nanos.is_none() && micros.is_none() && millis.is_none() && secs.is_none() && mins.is_none() && hours.is_none() && days.is_none() && weeks.is_none() {
                    return Err(de::Error::missing_field("One of `nanoseconds`, `microseconds`, `milliseconds`, `seconds`, `minutes`, `hours`, `days` or `weeks`"))
                }


                let mut nanos: u128 = nanos.unwrap_or(0u64) as u128;

                micros
                    .map(|v: u64| v as u128 )
                    .map(|v: u128| v * consts::NANOS_IN_A_MICROSECOND)
                    .inspect(|v| nanos += v);

                millis
                    .map(|v: u64| v as u128 )
                    .map(|v: u128| v * consts::NANOS_IN_A_MILLISECOND)
                    .inspect(|v| nanos += v);

                secs
                    .map(|v: u64| v as u128 )
                    .map(|v: u128| v * consts::NANOS_IN_A_SECOND)
                    .inspect(|v| nanos += v);

                mins
                    .map(|v: u64| v as u128 )
                    .map(|v: u128| v * consts::NANOS_IN_A_MINUTE)
                    .inspect(|v| nanos += v);

                hours
                    .map(|v: u64| v as u128 )
                    .map(|v: u128| v * consts::NANOS_IN_AN_HOUR)
                    .inspect(|v| nanos += v);

                days
                    .map(|v: u64| v as u128 )
                    .map(|v: u128| v * consts::NANOS_IN_A_DAY)
                    .inspect(|v| nanos += v);

                weeks
                    .map(|v: u64| v as u128 )
                    .map(|v: u128| v * consts::NANOS_IN_A_WEEK)
                    .inspect(|v| nanos += v);

                Ok(AbsoluteDuration {
                    nanos
                })
            }
        }

        const FIELDS: &[&str] = &["nanoseconds", "microseconds", "milliseconds", "seconds", "minutes", "hours", "days", "weeks"];
        deserializer.deserialize_struct("Duration", FIELDS, AbsoluteDurationVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::AbsoluteDuration;

    #[test]
    fn test_ns() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"nanoseconds":12}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 12);
    }

    #[test]
    fn test_us() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"microseconds":8}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 8000);
    }

    #[test]
    fn test_ms() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"milliseconds":89}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 89_000_000);
    }

    #[test]
    fn test_s() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"seconds":2}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 2_000_000_000);
    }

    #[test]
    fn test_m() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"minutes":3}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 180_000_000_000);
    }

    #[test]
    fn test_h() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"hours":4}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 14_400_000_000_000);
    }

    #[test]
    fn test_d() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"days":5}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 432_000_000_000_000);
    }

    #[test]
    fn test_w() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"weeks":1}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 604_800_000_000_000);
    }

    #[test]
    fn test_two() {
        let testee: AbsoluteDuration = serde_json::from_str(r#"{"nanoseconds":100, "seconds": 1}"#).expect("Unable to parse");

        assert_eq!(testee.nanos, 1_000_000_100);
    }

    #[test]
    fn test_none() {
        serde_json::from_str::<AbsoluteDuration>(r#"{}"#).expect_err("Somehow able to parse");
    }
}