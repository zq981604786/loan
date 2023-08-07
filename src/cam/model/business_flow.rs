use crate::cam::model::base::StrPnl;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use loan_macros::{GetStructFields,FieldsToVec};
use loan_trait::{GetStructFieldsTrait,ToVecTrait};

#[derive(Debug, Deserialize, Default, Serialize, PartialEq, Clone, GetStructFields, FieldsToVec)]
pub struct LoanBusinessFlow {
    loan_id: String,
    flow_id: String,
    business_type: String,
    // content: StrPnl,
    operator: String,
    operator_alias: String,
    company: String,
    department: String,
    operation_time: DateTime<Utc>,
}
