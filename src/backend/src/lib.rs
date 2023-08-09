use std::cell::RefCell;

use candid::export_service;
use canister_data::{CanisterData, InitArgs};

mod canister_data;
mod canister_lifecycle;
mod stable_memory_serializer_deserializer;
mod status;
#[cfg(test)]
mod test;
mod timer;

thread_local! {
    static CANISTER_DATA: RefCell<CanisterData> = RefCell::default();
}

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
