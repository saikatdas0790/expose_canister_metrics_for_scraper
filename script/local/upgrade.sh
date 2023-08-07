#!/usr/bin/env bash
set -euo pipefail

dfx build backend

cargo test

dfx canister install backend --mode upgrade --upgrade-unchanged --argument "(record {
    endpoint_to_initialize_with = opt \"https://webhook.site/8c5f2599-2906-420e-8e09-379da427e5d6\";
})"