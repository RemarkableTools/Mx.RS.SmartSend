#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SmartSendContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(addUser)]
    fn add_user(&self, user: ManagedAddress) {
        self.allowed_users().insert(user);
    }

    #[only_owner]
    #[endpoint(removeUser)]
    fn remove_user(&self, user: ManagedAddress) {
        self.allowed_users().swap_remove(&user);
    }

    #[payable("*")]
    #[endpoint(smartSend)]
    // EGLD / FungibleESDT / MetaESDT distribution
    fn token_distribution(
        &self,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>
    ) {
        let caller = self.blockchain().get_caller();
        require!(
            self.blockchain().get_owner_address() == caller || self.allowed_users().contains(&caller),
            "Caller is not allowed to use the contract"
        );

        let payment = self.call_value().egld_or_single_esdt();
        let mut sent_amount = BigUint::zero();

        for param in params.into_iter() {
            let (receiver, amount) = param.into_tuple();
            sent_amount += &amount;

            self.send().direct(&receiver, &payment.token_identifier, payment.token_nonce, &amount);
        }

        if payment.amount > sent_amount {
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
        let caller = self.blockchain().get_caller();
        require!(
            self.blockchain().get_owner_address() == caller || self.allowed_users().contains(&caller),
            "Caller is not allowed to use the contract"
        );

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
        let caller = self.blockchain().get_caller();
        require!(
            self.blockchain().get_owner_address() == caller || self.allowed_users().contains(&caller),
            "Caller is not allowed to use the contract"
        );

        let payment = self.call_value().single_esdt();
        let mut sent_amount = BigUint::zero();

        for param in params.into_iter() {
            let (receiver, amount) = param.into_tuple();
            sent_amount += &amount;

            self.send().direct_esdt(&receiver, &payment.token_identifier, payment.token_nonce, &amount);
        }

        if payment.amount > sent_amount {
            let remaining_amount = payment.amount - sent_amount;
            self.send().direct_esdt(&caller, &payment.token_identifier, payment.token_nonce, &remaining_amount);
        }
    }

//-------------------------------------------------------------------------------------------------------------------------
    #[view(getAllowedUsers)]
    #[storage_mapper("allowedUsers")]
    fn allowed_users(&self) -> UnorderedSetMapper<ManagedAddress>;
}
