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
        let mut sent_amount = BigUint::zero();

        for param in params.into_iter() {
            let (receiver, amount) = param.into_tuple();
            sent_amount += &amount;

            self.send().direct(&receiver, &payment.token_identifier, payment.token_nonce, &amount);
        }

        if payment.amount > sent_amount {
            let caller = self.blockchain().get_caller();
            let remaining_amount = payment.amount - sent_amount;
            self.send().direct(&caller, &payment.token_identifier, payment.token_nonce, &remaining_amount);
        }
    }

    #[payable("*")]
    #[endpoint(smartNftSend)]
    // NFT distribution
    fn nft_distribution(
        &self,
        params: MultiValueEncoded<MultiValue3<ManagedAddress, TokenIdentifier, u64>>
    ) {
        let payments = self.call_value().all_esdt_transfers();
        require!(
            params.len() == payments.len(),
            "Number of NFTs sent must be equal to number of transfers"
        );

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
        let mut sent_amount = BigUint::zero();

        for param in params.into_iter() {
            let (receiver, amount) = param.into_tuple();
            sent_amount += &amount;

            self.send().direct_esdt(&receiver, &payment.token_identifier, payment.token_nonce, &amount);
        }

        if payment.amount > sent_amount {
            let caller = self.blockchain().get_caller();
            let remaining_amount = payment.amount - sent_amount;
            self.send().direct_esdt(&caller, &payment.token_identifier, payment.token_nonce, &remaining_amount);
        }
    }
}