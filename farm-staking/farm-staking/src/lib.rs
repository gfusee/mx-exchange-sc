#![no_std]
#![allow(clippy::from_over_into)]
#![feature(trait_alias)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use base_impl_wrapper::FarmStakingWrapper;
use contexts::storage_cache::StorageCache;
use farm::base_functions::DoubleMultiPayment;
use farm_base_impl::base_traits_impl::FarmContract;
use fixed_supply_token::FixedSupplyToken;
use token_attributes::StakingFarmTokenAttributes;

use crate::custom_rewards::MAX_MIN_UNBOND_EPOCHS;

pub mod base_impl_wrapper;
pub mod claim_only_boosted_staking_rewards;
pub mod claim_stake_farm_rewards;
pub mod compound_stake_farm_rewards;
pub mod custom_rewards;
pub mod farm_token_roles;
pub mod stake_farm;
pub mod token_attributes;
pub mod unbond_farm;
pub mod unstake_farm;

pub const DEFAULT_FARM_POSITION_MIGRATION_NONCE: u64 = 1;

#[multiversx_sc::contract]
pub trait FarmStaking:
    custom_rewards::CustomRewardsModule
    + rewards::RewardsModule
    + config::ConfigModule
    + events::EventsModule
    + token_send::TokenSendModule
    + farm_token::FarmTokenModule
    + sc_whitelist_module::SCWhitelistModule
    + pausable::PausableModule
    + permissions_module::PermissionsModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + farm_base_impl::base_farm_init::BaseFarmInitModule
    + farm_base_impl::base_farm_validation::BaseFarmValidationModule
    + farm_base_impl::enter_farm::BaseEnterFarmModule
    + farm_base_impl::claim_rewards::BaseClaimRewardsModule
    + farm_base_impl::compound_rewards::BaseCompoundRewardsModule
    + farm_base_impl::exit_farm::BaseExitFarmModule
    + utils::UtilsModule
    + farm_token_roles::FarmTokenRolesModule
    + stake_farm::StakeFarmModule
    + claim_stake_farm_rewards::ClaimStakeFarmRewardsModule
    + compound_stake_farm_rewards::CompoundStakeFarmRewardsModule
    + unstake_farm::UnstakeFarmModule
    + unbond_farm::UnbondFarmModule
    + claim_only_boosted_staking_rewards::ClaimOnlyBoostedStakingRewardsModule
    + farm_boosted_yields::FarmBoostedYieldsModule
    + farm_boosted_yields::boosted_yields_factors::BoostedYieldsFactorsModule
    + week_timekeeping::WeekTimekeepingModule
    + weekly_rewards_splitting::WeeklyRewardsSplittingModule
    + weekly_rewards_splitting::events::WeeklyRewardsSplittingEventsModule
    + weekly_rewards_splitting::global_info::WeeklyRewardsGlobalInfo
    + weekly_rewards_splitting::locked_token_buckets::WeeklyRewardsLockedTokenBucketsModule
    + weekly_rewards_splitting::update_claim_progress_energy::UpdateClaimProgressEnergyModule
    + energy_query::EnergyQueryModule
{
    #[init]
    fn init(
        &self,
        farming_token_id: TokenIdentifier,
        division_safety_constant: BigUint,
        max_apr: BigUint,
        min_unbond_epochs: u64,
        owner: ManagedAddress,
        admins: MultiValueEncoded<ManagedAddress>,
    ) {
        // farming and reward token are the same
        self.base_farm_init(
            farming_token_id.clone(),
            farming_token_id,
            division_safety_constant,
            owner,
            admins,
        );

        require!(max_apr > 0u64, "Invalid max APR percentage");
        self.max_annual_percentage_rewards().set_if_empty(&max_apr);

        require!(
            min_unbond_epochs <= MAX_MIN_UNBOND_EPOCHS,
            "Invalid min unbond epochs"
        );
        self.min_unbond_epochs().set_if_empty(min_unbond_epochs);

        // Farm position migration code
        self.try_set_farm_position_migration_nonce();
    }

    #[payable("*")]
    #[endpoint(mergeFarmTokens)]
    fn merge_farm_tokens_endpoint(&self) -> DoubleMultiPayment<Self::Api> {
        let caller = self.blockchain().get_caller();
        self.migrate_old_farm_positions(&caller);

        let boosted_rewards = self.claim_only_boosted_payment(&caller);
        let boosted_rewards_payment = if boosted_rewards > 0 {
            EsdtTokenPayment::new(self.reward_token_id().get(), 0, boosted_rewards)
        } else {
            EsdtTokenPayment::new(self.reward_token_id().get(), 0, BigUint::zero())
        };

        let payments = self.get_non_empty_payments();
        let token_mapper = self.farm_token();
        let output_attributes: StakingFarmTokenAttributes<Self::Api> =
            self.merge_from_payments_and_burn(payments, &token_mapper);
        let new_token_amount = output_attributes.get_total_supply();

        let merged_farm_token = token_mapper.nft_create(new_token_amount, &output_attributes);
        self.send_payment_non_zero(&caller, &merged_farm_token);
        self.send_payment_non_zero(&caller, &boosted_rewards_payment);

        (merged_farm_token, boosted_rewards_payment).into()
    }

    #[view(calculateRewardsForGivenPosition)]
    fn calculate_rewards_for_given_position(
        &self,
        farm_token_amount: BigUint,
        attributes: StakingFarmTokenAttributes<Self::Api>,
    ) -> BigUint {
        self.require_queried();

        let mut storage_cache = StorageCache::new(self);
        FarmStakingWrapper::<Self>::generate_aggregated_rewards(self, &mut storage_cache);

        FarmStakingWrapper::<Self>::calculate_rewards(
            self,
            &ManagedAddress::zero(),
            &farm_token_amount,
            &attributes,
            &storage_cache,
        )
    }

    fn require_queried(&self) {
        let caller = self.blockchain().get_caller();
        let sc_address = self.blockchain().get_sc_address();
        require!(
            caller == sc_address,
            "May only call this function through VM query"
        );
    }

    fn try_set_farm_position_migration_nonce(&self) {
        if !self.farm_position_migration_nonce().is_empty() {
            return;
        }

        let farm_token_mapper = self.farm_token();

        let attributes = StakingFarmTokenAttributes {
            reward_per_share: BigUint::zero(),
            compounded_reward: BigUint::zero(),
            current_farm_amount: BigUint::zero(),
            original_owner: self.blockchain().get_sc_address(),
        };

        let migration_farm_token_nonce = if farm_token_mapper.get_token_state().is_set() {
            let migration_farm_token =
                farm_token_mapper.nft_create(BigUint::from(1u64), &attributes);
            farm_token_mapper.nft_burn(
                migration_farm_token.token_nonce,
                &migration_farm_token.amount,
            );
            migration_farm_token.token_nonce
        } else {
            DEFAULT_FARM_POSITION_MIGRATION_NONCE
        };

        self.farm_position_migration_nonce()
            .set(migration_farm_token_nonce);
    }
}
