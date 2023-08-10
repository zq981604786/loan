use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct BrokerLoanRecordFile {
    id: i64,
    created: DateTime<Utc>,
    updated: Option<DateTime<Utc>>,
    filename: String,
    operator: String,
    is_deleted: bool,
    loan_id: String,
    extra: BrokerLoanRecordFileExtra,
    company: String,
    department: String,
}

#[derive(Debug, Deserialize, Default)]
struct BrokerLoanRecordFileExtra {
    remark: String,
    alias: String,
    source: String,
    relation_id: String,
}
