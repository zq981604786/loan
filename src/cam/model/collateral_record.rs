// use crate::model::base::Pnl;
// use chrono::{DateTime, Utc};
// use rust_decimal::Decimal;
// use serde::{Deserialize, Serialize};
//
// #[derive(Debug, Deserialize, Default)]
// pub struct CollateralRecord {
//     id: i64,
//     loan_id: i64,
//     r#type: String,
//     created: DateTime<Utc>,
//     updated: DateTime<Utc>,
//     company: String,
//     counterparty: String,
//     operator: String,
//     collateral: Pnl,
//     repay_date: String,
//     department: String,
//     department_alias: String,
//     name: String,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct NonStandardCollateralParams {
//     more_info: String,
//     more_info_alias: String,
//     r#type: String,
//     amount: Decimal,
//     price: Decimal,
//     vu: Decimal,
//     currency: String,
//     unit_price: Decimal,
// }
