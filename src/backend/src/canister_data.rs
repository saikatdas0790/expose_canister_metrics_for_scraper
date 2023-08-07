use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Default, Serialize, Deserialize)]
pub struct CanisterData {
    pub rest_api_endpoint_to_send_canister_status_to: Option<String>,
}

#[derive(Deserialize, CandidType)]
pub struct InitArgs {
    pub endpoint_to_initialize_with: Option<String>,
}
