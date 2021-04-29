use crate::shared::naive_datetime_from_string;
use crate::shared::string_to_decimal;
use serde::Deserialize;
use serde::Serialize;
use chrono::naive::NaiveDateTime;
use rust_decimal::prelude::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticker {
    pub trade_id: i64,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub bid: Decimal,
    #[serde(with = "string_to_decimal")]
    pub ask: Decimal,
    #[serde(with = "string_to_decimal")]
    pub volume: Decimal,
    #[serde(with = "naive_datetime_from_string")]
    pub time: NaiveDateTime,
}