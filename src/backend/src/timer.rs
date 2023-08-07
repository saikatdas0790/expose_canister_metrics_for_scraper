use std::time::Duration;

use ic_cdk::api::management_canister::http_request::{
    self, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

use crate::CANISTER_DATA;

pub fn enqueue_timer_for_calling_metrics_rest_api() {
    ic_cdk_timers::set_timer_interval(Duration::from_secs(3600), || {
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

    let request_arg = CanisterHttpRequestArgument {
        url: url_to_ping,
        max_response_bytes: Some(0),
        method: HttpMethod::POST,
        headers: vec![HttpHeader {
            name: "canister-id".to_string(),
            value: ic_cdk::id().to_string(),
        }],
        ..Default::default()
    };

    http_request::http_request(request_arg, 1_000_000_000)
        .await
        .expect("Failed to ping");
}
