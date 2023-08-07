use crate::{
    canister_data::InitArgs, timer::enqueue_timer_for_calling_metrics_rest_api, CANISTER_DATA,
};

#[ic_cdk::init]
#[candid::candid_method(init)]
fn init(init_args: InitArgs) {
    if init_args.endpoint_to_initialize_with.is_none() {
        return;
    }

    CANISTER_DATA.with(|canister_data_ref_cell| {
        canister_data_ref_cell
            .borrow_mut()
            .rest_api_endpoint_to_send_canister_status_to = init_args.endpoint_to_initialize_with;
    });

    enqueue_timer_for_calling_metrics_rest_api();
}
