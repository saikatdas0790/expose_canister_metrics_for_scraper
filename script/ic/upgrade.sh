#!/usr/bin/env bash
set -euo pipefail

dfx build backend --network ic

cargo test

dfx canister install backend --mode upgrade --upgrade-unchanged --network ic --argument "(record {
    endpoint_to_initialize_with = opt \"https://receive-canister-metrics-and-push-to-timeseries-d-74gsa5ifla-uc.a.run.app/receive-metrics\";
})"
# dfx canister install backend --mode upgrade --upgrade-unchanged --network ic --argument "(record {
#     endpoint_to_initialize_with = opt \"https://amused-welcome-anemone.ngrok-free.app/receive-metrics\";
# })"