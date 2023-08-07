use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::{main, provisional::CanisterIdRecord};

#[derive(CandidType)]
pub struct CanisterStatus {
    cycle_balance: u128,
    memory_consumed: u128,
    idle_cycles_burned_per_day: u128,
}

#[derive(CandidType)]
pub struct CanisterStatusError {
    pub canister_id: Principal,
    pub error_message: String,
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn get_my_canister_cycle_balance_and_memory_usage(
) -> Result<CanisterStatus, CanisterStatusError> {
    let canister_status_result = main::canister_status(CanisterIdRecord {
        canister_id: ic_cdk::id(),
    })
    .await
    .map_err(|err| CanisterStatusError {
        canister_id: ic_cdk::id(),
        error_message: err.1,
    })?
    .0;

    Ok(CanisterStatus {
        cycle_balance: canister_status_result.cycles.0.try_into().unwrap(),
        memory_consumed: canister_status_result.memory_size.0.try_into().unwrap(),
        idle_cycles_burned_per_day: canister_status_result
            .idle_cycles_burned_per_day
            .0
            .try_into()
            .unwrap(),
    })
}
