use candid::CandidType;
use serde::Deserialize;
use types::ICP;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub new_storage_limit_bytes: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    SuccessNoChange,
    PaymentInsufficient(PaymentInsufficientResult),
    PaymentNotFound,
    StorageLimitExceeded(u64), // Returns the storage limit in bytes
    UserNotFound,
    InternalError(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct PaymentInsufficientResult {
    pub account_balance: ICP,
    pub amount_required: ICP,
}