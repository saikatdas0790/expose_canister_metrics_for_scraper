#!/usr/bin/env bash
set -euo pipefail

dfx canister create backend --no-wallet
dfx build backend

cargo test

dfx canister install backend --argument "(record {
    endpoint_to_initialize_with = opt \"http://init-portal.com:8000\";
})"