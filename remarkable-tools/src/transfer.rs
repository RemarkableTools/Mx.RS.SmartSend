use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait TransferModule:
    distribution::DistributionModule
{
    #[payable("*")]
    #[endpoint(smartSend)]
    fn smart_send(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        self.distribute_egld_or_esdt(params);
    }

    #[payable("*")]
    #[endpoint(smartNftSend)]
    fn smart_nft_send(
        &self,
        params: MultiValueEncoded<MultiValue3<ManagedAddress, TokenIdentifier, u64>>,
    ) {
        self.distribute_nfts(params);
    }
}
