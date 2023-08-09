#!/usr/bin/env bash
set -euo pipefail

dfx build backend

cargo test

dfx canister install backend --mode upgrade --upgrade-unchanged --argument "(record {
    endpoint_to_initialize_with = opt \"https://amused-welcome-anemone.ngrok-free.app/receive-metrics\";
})"