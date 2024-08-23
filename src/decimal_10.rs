use crate::{BoundSided, Normalizable, RangeBound};
use bigdecimal_04::BigDecimal;

use crate::{BoundSided, Normalizable, RangeBound};
use decimal_10::Decimal;

impl Normalizable for Decimal {
    fn normalize<S>(bound: RangeBound<S, Decimal>) -> RangeBound<S, Decimal>
    where
        S: BoundSided,
    {
        bound
    }
}

bounded_normalizable!(Decimal);
