use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum LoanStatus {
    accrual,
    settled,
    expired,
    running,
    closed,
    closing,
    reviewing,
    off,
    cancelled,
    deleted,
    pending,
}
