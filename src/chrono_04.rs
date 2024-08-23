use chrono_04::{DateTime, NaiveDate, NaiveDateTime, TimeZone};

use crate::{BoundSided, Normalizable, RangeBound};

impl<T> Normalizable for DateTime<T>
where
    T: TimeZone,
{
    fn normalize<S>(bound: RangeBound<S, DateTime<T>>) -> RangeBound<S, DateTime<T>>
    where
        S: BoundSided,
    {
        bound
    }
}

impl Normalizable for NaiveDateTime {
    fn normalize<S>(bound: RangeBound<S, NaiveDateTime>) -> RangeBound<S, NaiveDateTime>
    where
        S: BoundSided,
    {
        bound
    }
}

bounded_normalizable!(NaiveDate, ::chrono_04::Duration::days(1));
