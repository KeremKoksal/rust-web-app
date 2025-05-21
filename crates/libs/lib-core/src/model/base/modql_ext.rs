use std::str::FromStr;
use bigdecimal::{BigDecimal, ToPrimitive};
use sea_query::{Value, Nullable};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use crate::model::{Error, Result};

#[derive(Clone, Debug, FromRow, Type,  Serialize, Deserialize, PartialEq)]
#[sqlx(transparent)]
pub struct TheBigDecimal(pub BigDecimal);

impl From<TheBigDecimal> for Value {
    fn from(value: TheBigDecimal) -> Self {
        Value::Float(Some(value.0.to_f32().unwrap_or_default()))
    }
}

impl From<BigDecimal> for TheBigDecimal {
    fn from(value: BigDecimal) -> Self {
        TheBigDecimal(value)
    }
}
impl FromStr for TheBigDecimal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let big_decimal = BigDecimal::from_str(s)?;
        Ok(TheBigDecimal(big_decimal))
    }
}
impl Nullable for TheBigDecimal {
    fn null() -> Value {
        Value::Float(None)
    }
}