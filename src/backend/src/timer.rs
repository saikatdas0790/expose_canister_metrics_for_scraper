use std::time::Duration;

use ic_cdk::api::management_canister::http_request::{
    self, CanisterHttpRequestArgument, HttpMethod,
};
use rmp_serde::encode;

use crate::{status::get_my_canister_cycle_balance_and_memory_usage, CANISTER_DATA};

const PING_INTERVAL_FOR_CALLING_METRICS_REST_API: Duration = Duration::from_secs(5);

pub fn enqueue_timer_for_calling_metrics_rest_api() {
    ic_cdk_timers::set_timer_interval(PING_INTERVAL_FOR_CALLING_METRICS_REST_API, || {
        ic_cdk::spawn(ping_metrics_rest_api())
    });
}

async fn ping_metrics_rest_api() {
    let url_to_ping = CANISTER_DATA
        .with(|canister_data_ref_cell| {
            canister_data_ref_cell
                .borrow()
                .rest_api_endpoint_to_send_canister_status_to
                .clone()
        })
        .expect("Rest API endpoint is not set");

    let status = get_my_canister_cycle_balance_and_memory_usage().await;

    let request_arg = CanisterHttpRequestArgument {
        url: url_to_ping,
        max_response_bytes: Some(0),
        method: HttpMethod::POST,
        body: Some(encode::to_vec(&status).expect("Failed to serialize status")),
        ..Default::default()
    };

    http_request::http_request(request_arg, 1_000_000_000)
        .await
        .expect("Failed to ping");
}
