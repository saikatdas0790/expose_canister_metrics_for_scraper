use ic_cdk::api::call;

use crate::{
    canister_data::{CanisterData, InitArgs},
    stable_memory_serializer_deserializer,
    timer::enqueue_timer_for_calling_metrics_rest_api,
    CANISTER_DATA,
};

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    restore_data_from_stable_memory();
    set_init_arguments();
    enqueue_timer_for_calling_metrics_rest_api();
}

fn restore_data_from_stable_memory() {
    match stable_memory_serializer_deserializer::deserialize_from_stable_memory::<CanisterData>() {
        Ok(canister_data) => {
            CANISTER_DATA.with(|canister_data_ref_cell| {
                *canister_data_ref_cell.borrow_mut() = canister_data;
            });
        }
        Err(e) => {
            ic_cdk::print(format!("Error: {:?}", e));
            panic!("Failed to restore canister data from stable memory");
        }
    }
}

fn set_init_arguments() {
    let (init_args,): (InitArgs,) = call::arg_data();

    if init_args.endpoint_to_initialize_with.is_some() {
        CANISTER_DATA.with(|canister_data_ref_cell| {
            canister_data_ref_cell
                .borrow_mut()
                .rest_api_endpoint_to_send_canister_status_to =
                init_args.endpoint_to_initialize_with;
        });
    }
}
