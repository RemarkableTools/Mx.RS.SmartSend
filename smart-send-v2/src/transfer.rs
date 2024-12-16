use multiversx_sc::imports::*;

use crate::{data, users};
use crate::storage;
use data::Transfer;

#[multiversx_sc::module]
pub trait TransferModule:
    distribution::DistributionModule
    + users::UsersModule
    + storage::StorageModule
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
            transfers.len() < 1500,
            "The number of transfers should be lower than 1500"
        );

        let payments = self.call_value().all_esdt_transfers();
        require!(
            payments.len() == 2,
            "Invalid number of payments received"
        );

        let wegld = self.wegld_identifier().get();
        let payment_egld = payments.get(0).as_refs().to_owned_payment();
        require!(
            payment_egld.token_identifier == wegld,
            "First payments should be {}", wegld
        );

        let payment_esdt = payments.get(1).as_refs().to_owned_payment();
        let transfers_list = self.store_transfers(caller.clone(), payment_esdt.clone(), transfers);

        self.tx()
            .to(executor_address.clone())
            .payment(payment_egld)
            .transfer();

        let transfer_id: u64;
        if self.initiator_transfers(&caller).contains(&executor_address) {
            transfer_id = self.executor_transfers(&executor_address).get();
        } else {
            let mut rand_source = RandomnessSource::new();
            transfer_id = rand_source.next_u64();

            self.initiator_transfers(&caller).insert(executor_address.clone());
            self.executor_transfers(&executor_address).set(transfer_id);
        }

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
        opt_transfers_id: OptionalValue<u64>,
    ) {
        let transfers_id = match opt_transfers_id {
            OptionalValue::Some(id) => id,
            OptionalValue::None => self.executor_transfers(&self.blockchain().get_caller()).get(),
        };

        let transfers = self.tokens_transfers(transfers_id);
        if transfers.is_empty() {
            sc_panic!("No transfers found for id {}", transfers_id);
        }

        for transfer in transfers.iter().take(100) {
            self.tx()
                .to(&transfer.receiver)
                .payment(&transfer.payment_token)
                .transfer();

            self.tokens_transfers(transfers_id).swap_remove(&transfer);
        }
    }

    #[endpoint(finishExecution)]
    fn smart_finish(
        &self,
        initiator_address: ManagedAddress,
        executor_address: ManagedAddress,
    ) {
        let transfers_id = self.executor_transfers(&executor_address).get();
        let transfers_count = self.tokens_transfers(transfers_id).len();
        require!(
             transfers_count == 0,
            "Cannot finish execution, there are {} transfers left to be executed", transfers_count
        );

        self.executor_transfers(&executor_address).clear();
        self.initiator_transfers(&initiator_address).swap_remove(&executor_address);
    }

    #[view(getInitiatorTransfers)]
    #[storage_mapper("initiatorTransfers")]
    fn initiator_transfers(&self, initiator_address: &ManagedAddress) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getExecutorTransfers)]
    #[storage_mapper("executorTransfers")]
    fn executor_transfers(&self, executor_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getTokensTransfers)]
    #[storage_mapper("tokensTransfers")]
    fn tokens_transfers(&self, transfers_id: u64) -> UnorderedSetMapper<Transfer<Self::Api>>;
}
