multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait SmartSendModule {
    #[payable("*")]
    #[endpoint(smartSend)]
    // EGLD / FungibleESDT / MetaESDT distribution
    fn token_distribution(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>
    ) {
        let payment = self.call_value().egld_or_single_esdt();

        for param in params.into_iter() {
            let (receiver, amount) = param.into_tuple();
            self.send().direct(&receiver, &payment.token_identifier, payment.token_nonce, &amount);
        }
    }

    #[payable("*")]
    #[endpoint(smartNftSend)]
    // NFT distribution
    fn nft_distribution(
        &self,
        params: MultiValueEncoded<MultiValue3<ManagedAddress, TokenIdentifier, u64>>
    ) {
        for param in params.into_iter() {
            let (receiver, token_identifier, nonce) = param.into_tuple();

            self.send().direct_esdt(&receiver, &token_identifier, nonce, &BigUint::from(1u64));
        }
    }

    #[payable("*")]
    #[endpoint(smartSftSend)]
    // SFT distribution
    fn sft_distribution(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>
    ) {
        let payment = self.call_value().single_esdt();

        for param in params.into_iter() {
            let (receiver, amount) = param.into_tuple();

            self.send().direct_esdt(&receiver, &payment.token_identifier, payment.token_nonce, &amount);
        }
    }
}