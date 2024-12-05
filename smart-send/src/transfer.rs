use multiversx_sc::imports::*;
use crate::users;

#[multiversx_sc::module]
pub trait TransferModule:
    distribution::DistributionModule
    + users::UsersModule
{
    #[payable("*")]
    #[endpoint(smartSend)]
    fn smart_send(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        self.require_user_is_allowed(self.blockchain().get_caller());

        self.distribute_egld_or_esdt(params);
    }

    #[payable("*")]
    #[endpoint(smartNftSend)]
    fn smart_nft_send(
        &self,
        params: MultiValueEncoded<MultiValue3<ManagedAddress, TokenIdentifier, u64>>,
    ) {
        self.require_user_is_allowed(self.blockchain().get_caller());

        self.distribute_nfts(params);
    }
}
