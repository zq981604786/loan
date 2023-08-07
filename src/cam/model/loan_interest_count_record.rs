use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use loan_macros::{GetStructFields,FieldsToVec};
use loan_trait::{GetStructFieldsTrait,ToVecTrait};

#[derive(Debug, Deserialize, Default, Serialize,PartialEq,Clone,GetStructFields,FieldsToVec)]
pub struct LoanInterestCountRecord {
    pub id: i64,
    rsc_name: String,
    loan_id: String,
    asset_base: String,
    principal: Decimal,
    counting_date: String,
    counting_time: String,
    counting_interest: Decimal,
    counting_interest_value: Decimal,
    counting_penalty: Decimal,
    counting_penalty_value: Decimal,
    currency_rate: Decimal,
    currency_rate_source: String,
    valuation_currency: String,
    price: Decimal,
    company: String,
    department: String,
    remark: String,
    r#type: String,
    status: String,
}
