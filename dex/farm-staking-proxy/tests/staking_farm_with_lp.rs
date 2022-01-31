use elrond_wasm_debug::{rust_biguint, testing_framework::BlockchainStateWrapper};

pub mod constants;
pub mod staking_farm_with_lp_external_contracts;
pub mod staking_farm_with_lp_staking_contract_interactions;

use constants::*;
use staking_farm_with_lp_external_contracts::*;
use staking_farm_with_lp_staking_contract_interactions::*;

#[test]
fn test_all_setup() {
    let rust_zero = rust_biguint!(0u64);
    let mut wrapper = BlockchainStateWrapper::new();
    let owner_addr = wrapper.create_user_account(&rust_zero);
    let user_addr = wrapper.create_user_account(&rust_biguint!(100_000_000));

    let pair_wrapper = setup_pair(&owner_addr, &user_addr, &mut wrapper, pair::contract_obj);
    let lp_farm_wrapper = setup_lp_farm(
        &owner_addr,
        &user_addr,
        &mut wrapper,
        farm::contract_obj,
        USER_TOTAL_LP_TOKENS,
    );
    let staking_farm_wrapper =
        setup_staking_farm(&owner_addr, &mut wrapper, farm_staking::contract_obj);
    let _proxy_wrapper = setup_proxy(
        &owner_addr,
        lp_farm_wrapper.address_ref(),
        staking_farm_wrapper.address_ref(),
        pair_wrapper.address_ref(),
        &mut wrapper,
        farm_staking_proxy::contract_obj,
    );
}
