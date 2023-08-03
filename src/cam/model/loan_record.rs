use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use crate::cam::model::loan_interest_count_record::LoanInterestCountRecord;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoanRecordVM {
    pub ltv_rate: Decimal, // 使用#[serde(rename = "ltv_rate")]属性将ltv_rate字段与JSON中的ltv_rate字段进行映射。
    pub stablecoin_decoupling_ltv_rate: Decimal,
    payable_interest: Decimal,
    payable_penalty: Decimal,

    #[serde(default)]
    pub interest_count_records: Vec<LoanInterestCountRecord>,
}