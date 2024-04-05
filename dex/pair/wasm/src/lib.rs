// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           55
// Async Callback (empty):               1
// Total number of exported functions:  57

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    pair
    (
        init => init
        upgrade => upgrade
        setLpTokenIdentifier => set_lp_token_identifier
        getFeeState => is_fee_enabled
        whitelist => whitelist_endpoint
        removeWhitelist => remove_whitelist
        addTrustedSwapPair => add_trusted_swap_pair
        removeTrustedSwapPair => remove_trusted_swap_pair
        setupFeesCollector => setup_fees_collector
        setFeeOn => set_fee_on
        getFeeDestinations => get_fee_destinations
        getTrustedSwapPairs => get_trusted_swap_pairs
        getWhitelistedManagedAddresses => get_whitelisted_managed_addresses
        getFeesCollectorAddress => fees_collector_address
        getFeesCollectorCutPercentage => fees_collector_cut_percentage
        setStateActiveNoSwaps => set_state_active_no_swaps
        setFeePercents => set_fee_percent
        getLpTokenIdentifier => get_lp_token_identifier
        getTotalFeePercent => total_fee_percent
        getSpecialFee => special_fee_percent
        getRouterManagedAddress => router_address
        getFirstTokenId => first_token_id
        getSecondTokenId => second_token_id
        getTotalSupply => lp_token_supply
        getInitialLiquidtyAdder => initial_liquidity_adder
        getReserve => pair_reserve
        getSafePriceCurrentIndex => safe_price_current_index
        updateAndGetTokensForGivenPositionWithSafePrice => update_and_get_tokens_for_given_position_with_safe_price
        updateAndGetSafePrice => update_and_get_safe_price
        setLockingDeadlineEpoch => set_locking_deadline_epoch
        setLockingScAddress => set_locking_sc_address
        setUnlockEpoch => set_unlock_epoch
        getLockingScAddress => locking_sc_address
        getUnlockEpoch => unlock_epoch
        getLockingDeadlineEpoch => locking_deadline_epoch
        addAdmin => add_admin_endpoint
        removeAdmin => remove_admin_endpoint
        updateOwnerOrAdmin => update_owner_or_admin_endpoint
        getPermissions => permissions
        addToPauseWhitelist => add_to_pause_whitelist
        removeFromPauseWhitelist => remove_from_pause_whitelist
        pause => pause
        resume => resume
        getState => state
        addInitialLiquidity => add_initial_liquidity
        addLiquidity => add_liquidity
        removeLiquidity => remove_liquidity
        removeLiquidityAndBuyBackAndBurnToken => remove_liquidity_and_burn_token
        swapNoFeeAndForward => swap_no_fee
        swapTokensFixedInput => swap_tokens_fixed_input
        swapTokensFixedOutput => swap_tokens_fixed_output
        getTokensForGivenPosition => get_tokens_for_given_position
        getReservesAndTotalSupply => get_reserves_and_total_supply
        getAmountOut => get_amount_out_view
        getAmountIn => get_amount_in_view
        getEquivalent => get_equivalent
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
