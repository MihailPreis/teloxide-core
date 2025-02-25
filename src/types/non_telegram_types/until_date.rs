use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{de::Visitor, Deserialize, Serialize};

/// A range of time, before some date (for example a time before a restrictions
/// will be lifted from a member of a chat).
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum UntilDate {
    /// The range is bound by a given date and time.
    Date(DateTime<Utc>),
    /// There is no end date, the range is unbounded.
    Forever,
}

impl<'de> Deserialize<'de> for UntilDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UntilDateVisitor;

        impl<'v> Visitor<'v> for UntilDateVisitor {
            type Value = UntilDate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer representing a UNIX timestamp or a 0")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0 => Ok(UntilDate::Forever),
                    timestamp => Ok(UntilDate::Date(DateTime::from_utc(
                        NaiveDateTime::from_timestamp(timestamp, 0),
                        Utc,
                    ))),
                }
            }
        }

        deserializer.deserialize_i64(UntilDateVisitor)
    }
}

impl Serialize for UntilDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(match self {
            UntilDate::Date(dt) => dt.timestamp(),
            UntilDate::Forever => 0,
        })
    }
}
