use multiversx_sc::imports::*;
use multiversx_sc::{
    api::ManagedTypeApi,
    codec,
    derive::{type_abi},
    proxy_imports::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{ManagedAddress},
};
use crate::users;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, PartialEq, Eq, Debug)]
pub struct Transfer<M: ManagedTypeApi> {
    pub receiver: ManagedAddress<M>,
    pub payment_token: EsdtTokenPayment<M>,
}

impl<M: ManagedTypeApi> Transfer<M> {
    pub fn new(receiver: ManagedAddress<M>, payment_token: EsdtTokenPayment<M>) -> Self {
        Transfer {
            receiver,
            payment_token,
        }
    }
}

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

    #[payable("*")]
    #[endpoint(smartSave)]
    fn smart_save(
        &self,
        executor_address: ManagedAddress,
        transfers: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        let caller = self.blockchain().get_caller();
        self.require_user_is_allowed(caller.clone());

        require!(
            transfers.len() < 1400,
            "The number of transfers should be lower than 1400"
        );

        let payments = self.call_value().all_esdt_transfers();
        // require payments.len == 2

        let payment_egld = payments.get(0).as_refs().to_owned_payment();
        // require first payment is EGLD

        let payment_esdt = payments.get(1).as_refs().to_owned_payment();
        let transfers_list = self.store_transfers(caller.clone(), payment_esdt.clone(), transfers);

        self.tx()
            .to(executor_address.clone())
            .payment(payment_egld)
            .transfer();

        let mut rand_source = RandomnessSource::new();
        let transfer_id = rand_source.next_u64();
        self.initiator_transfers(&caller).insert(transfer_id);
        self.tokens_transfers(transfer_id).extend(transfers_list.into_iter());
    }

    fn store_transfers(
        &self,
        caller: ManagedAddress,
        payment: EsdtTokenPayment,
        transfers: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) -> MultiValueEncoded<Transfer<Self::Api>> {
        let mut transfers_list: MultiValueEncoded<Self::Api, Transfer<Self::Api>> = MultiValueEncoded::new();
        let mut total_amount = BigUint::zero();

        for transfer in transfers.into_iter() {
            let (receiver, amount) = transfer.into_tuple();
            total_amount += &amount;
            if total_amount > payment.amount {
                sc_panic!("Payment received cannot satisfy all the transfers");
            }

            let token = EsdtTokenPayment::new(payment.token_identifier.clone(), payment.token_nonce, amount);
            transfers_list.push(Transfer::new(receiver, token));
        }

        if payment.amount > total_amount.clone() {
            let remaining_amount = payment.amount - total_amount.clone();
            self.tx()
                .to(caller)
                .payment(EsdtTokenPayment::new(payment.token_identifier.clone(), payment.token_nonce, remaining_amount))
                .transfer();
        }

        transfers_list
    }

    #[endpoint(smartExecute)]
    fn smart_execute(
        &self,
        transfers_id: u64,
    ) {
        let transfers = self.tokens_transfers(transfers_id);
        for transfer in transfers.iter().take(100) {
            self.tx()
                .to(&transfer.receiver)
                .payment(&transfer.payment_token)
                .transfer();

            self.tokens_transfers(transfers_id).swap_remove(&transfer);
        }
    }

    #[view(getInitiatorTransfers)]
    #[storage_mapper("initiatorTransfers")]
    fn initiator_transfers(&self, initiator_address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getTokensTransfers)]
    #[storage_mapper("tokensTransfers")]
    fn tokens_transfers(&self, transfers_id: u64) -> UnorderedSetMapper<Transfer<Self::Api>>;
}
