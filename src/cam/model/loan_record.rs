use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use loan_macros::{GetStructFields,FieldsToVec};
use loan_trait::{GetStructFieldsTrait,ToVecTrait};

use crate::cam::model::{
    loan_interest_count_record::LoanInterestCountRecord,
    business_flow::LoanBusinessFlow,
    base::StrPnl,
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
    pub borrower: String,
    borrower_name: String,
    borrower_alias: String,
    pub days: i64,
    pub calculation: String,
    pub year_days: i64,
    commission: Decimal,
    commission_type: String,
    payable_commission_amount: Decimal,
    paid_commission_amount: Decimal,
    comment: String,
    memo: String,
    pub option: String,
    daily_interest_rate: Decimal,
    daily_penalty_rate: Decimal,
    prepay_interest_days: i64,

    collateral_account: String,
    collateral_account_alias: String,
    pub repay_type: String,
    repay_counts: Decimal,
    prepay_interest: Decimal,
    // force_close_info: CAMJSON,
    initial_ltv: String,
    currency_map: StrPnl,
    timezone: String,
    estimated_interest: Decimal,
    latest_estimated_interest: Decimal,
    initial_collateral_vu: Decimal,

    ltv_with_interest: bool,
    monthly_repay_date: String,
    match_record_operation: String,

    match_record_time: i64,
    contract_number: String,

    // non_standard_collaterals: NonStandardCollateralMap,
    non_standard_collaterals_vu: Decimal,
    // initial_non_standard_collaterals: NonStandardCollateralMap,
    initial_non_standard_collaterals_vu: Decimal,

    interval_type: String,
    interval_num: String,

    lender_customer_type: String,
    borrower_customer_type: String,
}
