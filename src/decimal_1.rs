use std::str::FromStr;

use crate::{BoundSided, Normalizable, RangeBound};
use decimal_1::Decimal;

bounded_normalizable!(Decimal, ::decimal_1::Decimal::from_str("1.0").unwrap());
