use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use loan_macros::{GetStructFields,FieldsToVec};
use loan_trait::{GetStructFieldsTrait,ToVecTrait};

use crate::cam::model::{
    loan_interest_count_record::LoanInterestCountRecord,
    business_flow::LoanBusinessFlow,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoanRecordVM {
    #[serde(flatten)] //将LoanRecord结构体的字段展开到LoanRecordVM中。这样可以实现结构体字段的嵌套。
    pub loan_record: LoanRecord,

    pub ltv_rate: Decimal, // 使用#[serde(rename = "ltv_rate")]属性将ltv_rate字段与JSON中的ltv_rate字段进行映射。
    pub stablecoin_decoupling_ltv_rate: Decimal,
    payable_interest: Decimal,
    payable_penalty: Decimal,

    #[serde(default)]
    pub interest_count_records: Vec<LoanInterestCountRecord>,

    #[serde(default)]
    pub business_flows:Vec<LoanBusinessFlow>,
}



#[derive(Debug, Deserialize, Default, Serialize, PartialEq, Clone)]
pub struct LoanRecord{
    pub id:i64,
    pub r#type: String,
    pub extra: ExtraParams,
}

#[derive(Debug, Deserialize, Default, Serialize, PartialEq, Clone)]
pub struct ExtraParams {
    pub repay_type: String,
}
