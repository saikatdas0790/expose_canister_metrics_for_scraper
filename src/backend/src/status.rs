use candid::Principal;
use ic_cdk::api::management_canister::{main, provisional::CanisterIdRecord};
use serde::Serialize;

#[derive(Serialize)]
pub struct CanisterStatus {
    pub canister_id: Principal,
    cycle_balance: i64,
    idle_cycles_burned_per_day: i64,
    memory_size: i64,
}

#[derive(Serialize)]
pub struct CanisterStatusError {
    pub canister_id: Principal,
    pub error_message: String,
}

pub async fn get_my_canister_cycle_balance_and_memory_usage(
) -> Result<CanisterStatus, CanisterStatusError> {
    let canister_id = ic_cdk::id();

    let canister_status_result = main::canister_status(CanisterIdRecord { canister_id })
        .await
        .map_err(|err| CanisterStatusError {
            canister_id: ic_cdk::id(),
            error_message: err.1,
        })?
        .0;

    Ok(CanisterStatus {
        canister_id,
        cycle_balance: canister_status_result.cycles.0.try_into().unwrap(),
        memory_size: canister_status_result.memory_size.0.try_into().unwrap(),
        idle_cycles_burned_per_day: canister_status_result
            .idle_cycles_burned_per_day
            .0
            .try_into()
            .unwrap(),
    })
}
