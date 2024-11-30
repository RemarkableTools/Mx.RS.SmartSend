use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait TransferModule: distribution::DistributionModule {
    #[payable("*")]
    #[endpoint(smartSend)]
    fn token_distribution(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        self.distribute_egld_or_esdt(self.blockchain().get_caller(), params);
    }

    #[payable("*")]
    #[endpoint(smartNftSend)]
    fn nft_distribution(
        &self,
        params: MultiValueEncoded<MultiValue3<ManagedAddress, TokenIdentifier, u64>>,
    ) {
        self.distribute_nfts(params);
    }

    #[payable("*")]
    #[endpoint(smartSftSend)]
    fn sft_distribution(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        self.distribute_sft(self.blockchain().get_caller(), params);
    }
}