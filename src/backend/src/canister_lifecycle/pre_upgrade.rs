use crate::{stable_memory_serializer_deserializer, CANISTER_DATA};

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    CANISTER_DATA.with(|canister_data_ref_cell| {
        let canister_data = canister_data_ref_cell.take();
        stable_memory_serializer_deserializer::serialize_to_stable_memory(canister_data)
            .expect("Failed to serialize canister data");
    });
}
