use crate::{BoundSided, Normalizable, RangeBound};
use bigdecimal_04::BigDecimal;

impl Normalizable for Decimal {
    fn normalize<S>(bound: RangeBound<S, Decimal>) -> RangeBound<S, Decimal>
    where
        S: BoundSided,
    {
        bound
    }
}

bounded_normalizable!(Decimal);
