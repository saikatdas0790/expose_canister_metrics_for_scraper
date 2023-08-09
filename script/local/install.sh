#!/usr/bin/env bash
set -euo pipefail

dfx canister create backend --no-wallet
dfx build backend

cargo test

dfx canister install backend --argument "(record {
    endpoint_to_initialize_with = opt \"https://amused-welcome-anemone.ngrok-free.app/receive-metrics\";
})"