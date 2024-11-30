use multiversx_sc::imports::*;
use crate::users;

#[multiversx_sc::module]
pub trait TransferModule:
    users::UsersModule +
    distribution::DistributionModule
{
    #[payable("*")]
    #[endpoint(smartSend)]
    fn token_distribution(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        let caller = self.blockchain().get_caller();
        self.require_user_is_allowed(caller.clone());

        self.distribute_egld_or_esdt(caller, params);
    }

    #[payable("*")]
    #[endpoint(smartNftSend)]
    fn nft_distribution(
        &self,
        params: MultiValueEncoded<MultiValue3<ManagedAddress, TokenIdentifier, u64>>,
    ) {
        let caller = self.blockchain().get_caller();
        self.require_user_is_allowed(caller);

        self.distribute_nfts(params);
    }

    #[payable("*")]
    #[endpoint(smartSftSend)]
    fn sft_distribution(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        let caller = self.blockchain().get_caller();
        self.require_user_is_allowed(caller.clone());

        self.distribute_sft(caller, params);
    }
}
