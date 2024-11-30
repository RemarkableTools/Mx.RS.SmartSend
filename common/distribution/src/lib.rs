#![no_std]

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait DistributionModule {
    fn distribute_egld_or_esdt(
        &self,
        caller: ManagedAddress,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        let payment = self.call_value().egld_or_single_esdt();
        let mut sent_amount = BigUint::zero();
        for param in params.into_iter() {
            let (receiver, amount) = param.into_tuple();
            sent_amount += &amount;

            self.tx()
                .to(receiver)
                .payment(EgldOrEsdtTokenPayment::new(payment.token_identifier.clone(), payment.token_nonce, amount))
                .transfer();
        }

        if payment.amount > sent_amount {
            let remaining_amount = payment.amount - sent_amount;
            self.tx()
                .to(caller)
                .payment(EgldOrEsdtTokenPayment::new(payment.token_identifier, payment.token_nonce, remaining_amount))
                .transfer();
        }
    }

    fn distribute_nfts(
        &self,
        params: MultiValueEncoded<MultiValue3<ManagedAddress, TokenIdentifier, u64>>,
    ) {
        let payments = self.call_value().all_esdt_transfers();
        require!(
            params.len() == payments.len(),
            "Number of NFTs sent must be equal to number of transfers"
        );

        for param in params.into_iter() {
            let (receiver, token_identifier, nonce) = param.into_tuple();

            self.tx()
                .to(receiver)
                .payment(EsdtTokenPayment::new(token_identifier, nonce, BigUint::from(1u64)))
                .transfer();
        }
    }

    fn distribute_sft(
        &self,
        caller: ManagedAddress,
        params: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        let payment = self.call_value().single_esdt();
        let mut sent_amount = BigUint::zero();
        for param in params.into_iter() {
            let (receiver, amount) = param.into_tuple();
            sent_amount += &amount;

            self.tx()
                .to(receiver)
                .payment(EsdtTokenPayment::new(payment.token_identifier.clone(), payment.token_nonce, amount))
                .transfer();
        }

        if payment.amount > sent_amount {
            let remaining_amount = payment.amount - sent_amount;
            self.tx()
                .to(caller)
                .payment(EsdtTokenPayment::new(payment.token_identifier, payment.token_nonce, remaining_amount))
                .transfer();
        }
    }
}
