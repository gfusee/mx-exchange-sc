// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           31
// Async Callback:                       1
// Total number of exported functions:  33

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    router
    (
        init => init
        pause => pause
        resume => resume
        createPair => create_pair_endpoint
        upgradePair => upgrade_pair_endpoint
        issueLpToken => issue_lp_token
        setLocalRoles => set_local_roles
        setLocalRolesOwner => set_local_roles_owner
        removePair => remove_pair
        setFeeOn => set_fee_on
        setFeeOff => set_fee_off
        setPairCreationEnabled => set_pair_creation_enabled
        migratePairMap => migrate_pair_map
        getPairCreationEnabled => pair_creation_enabled
        getState => state
        getOwner => owner
        getAllPairsManagedAddresses => get_all_pairs_addresses
        getAllPairTokens => get_all_token_pairs
        getAllPairContractMetadata => get_all_pair_contract_metadata
        getPair => get_pair
        clearPairTemporaryOwnerStorage => clear_pair_temporary_owner_storage
        setTemporaryOwnerPeriod => set_temporary_owner_period
        setPairTemplateAddress => set_pair_template_address
        getPairTemplateAddress => pair_template_address
        getTemporaryOwnerPeriod => temporary_owner_period
        multiPairSwap => multi_pair_swap
        configEnableByUserParameters => config_enable_by_user_parameters
        addCommonTokensForUserPairs => add_common_tokens_for_user_pairs
        removeCommonTokensForUserPairs => remove_common_tokens_for_user_pairs
        setSwapEnabledByUser => set_swap_enabled_by_user
        getEnableSwapByUserConfig => try_get_config
        getCommonTokensForUserPairs => common_tokens_for_user_pairs
    )
}

multiversx_sc_wasm_adapter::async_callback! { router }
