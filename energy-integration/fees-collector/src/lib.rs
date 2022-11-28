#![no_std]

elrond_wasm::imports!();

use common_types::{PaymentsVec, Week};
use core::marker::PhantomData;
use energy_factory::locked_token_transfer::ProxyTrait as _;
use energy_query::Energy;
use weekly_rewards_splitting::base_impl::WeeklyRewardsSplittingTraitsModule;

pub mod config;
pub mod events;
pub mod fees_accumulation;

#[elrond_wasm::contract]
pub trait FeesCollector:
    config::ConfigModule
    + events::FeesCollectorEventsModule
    + weekly_rewards_splitting::WeeklyRewardsSplittingModule
    + weekly_rewards_splitting::events::WeeklyRewardsSplittingEventsModule
    + weekly_rewards_splitting::global_info::WeeklyRewardsGlobalInfo
    + weekly_rewards_splitting::locked_token_buckets::WeeklyRewardsLockedTokenBucketsModule
    + weekly_rewards_splitting::update_claim_progress_energy::UpdateClaimProgressEnergyModule
    + fees_accumulation::FeesAccumulationModule
    + locking_module::lock_with_energy_module::LockWithEnergyModule
    + energy_query::EnergyQueryModule
    + week_timekeeping::WeekTimekeepingModule
    + elrond_wasm_modules::pause::PauseModule
    + utils::UtilsModule
{
    #[init]
    fn init(&self, locked_token_id: TokenIdentifier, energy_factory_address: ManagedAddress) {
        let current_epoch = self.blockchain().get_block_epoch();
        self.first_week_start_epoch().set_if_empty(current_epoch);
        self.require_valid_token_id(&locked_token_id);
        self.require_sc_address(&energy_factory_address);

        let mut tokens = MultiValueEncoded::new();
        tokens.push(locked_token_id.clone());
        self.add_known_tokens(tokens);

        self.locked_token_id().set(locked_token_id);
        self.energy_factory_address().set(&energy_factory_address);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) -> PaymentsVec<Self::Api> {
        require!(self.not_paused(), "Cannot claim while paused");

        let caller = self.blockchain().get_caller();
        let wrapper = FeesCollectorWrapper::new();
        let rewards = self.claim_multi(&wrapper, &caller);
        let mut all_rewards = ManagedVec::new();
        if !rewards.is_empty() {
            let locked_token_id = self.locked_token_id().get();
            let mut fungible_rewards = ManagedVec::new();
            for reward in &rewards {
                if reward.token_identifier == locked_token_id {
                    let energy_factory_addr = self.energy_factory_address().get();
                    let locked_rewards = self.lock_virtual(
                        self.get_base_token_id(&energy_factory_addr),
                        reward.amount.clone(),
                        caller.clone(),
                        caller.clone(),
                    );
                    all_rewards.push(locked_rewards);
                } else {
                    fungible_rewards.push(EsdtTokenPayment::new(
                        reward.token_identifier,
                        reward.token_nonce,
                        reward.amount,
                    ));
                }
            }
            if !fungible_rewards.is_empty() {
                self.send().direct_multi(&caller, &fungible_rewards);
                all_rewards.append_vec(fungible_rewards);
            }
        }

        all_rewards
    }

    fn set_energy_in_factory(&self, user: ManagedAddress, energy: Energy<Self::Api>) {
        let sc_address = self.energy_factory_address().get();
        let _: () = self
            .energy_factory_proxy(sc_address)
            .set_user_energy_after_locked_token_transfer(user, energy)
            .execute_on_dest_context();
    }
}

pub struct FeesCollectorWrapper<T: FeesCollector> {
    phantom: PhantomData<T>,
}

impl<T: FeesCollector> Default for FeesCollectorWrapper<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FeesCollector> FeesCollectorWrapper<T> {
    pub fn new() -> FeesCollectorWrapper<T> {
        FeesCollectorWrapper {
            phantom: PhantomData,
        }
    }
}

impl<T> WeeklyRewardsSplittingTraitsModule for FeesCollectorWrapper<T>
where
    T: FeesCollector,
{
    type WeeklyRewardsSplittingMod = T;

    fn collect_rewards_for_week(
        &self,
        module: &Self::WeeklyRewardsSplittingMod,
        week: Week,
    ) -> PaymentsVec<<Self::WeeklyRewardsSplittingMod as ContractBase>::Api> {
        module.collect_and_clear_all_accumulated_fees_for_week(week)
    }
}
