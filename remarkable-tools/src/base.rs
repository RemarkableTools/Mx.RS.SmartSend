#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod smart_send;
pub mod storage;
use core::ops::Deref;

#[multiversx_sc::contract]
pub trait RemarkableToolsSmartSend:
    smart_send::SmartSendModule +
    storage::StorageModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(generate)]
    fn generate(&self) -> ManagedAddress {
        let egld_value = self.call_value().egld_value();
        let payment_amount = egld_value.deref();
        require!(
            payment_amount == &self.contract_price().get(),
            "Invalid EGLD payment amount"
        );

        let args: ManagedArgBuffer<Self::Api> = ManagedArgBuffer::new();
        let (contract_address, _) = self.send_raw().deploy_from_source_contract(
            self.blockchain().get_gas_left()/2,
            &BigUint::zero(),
            &self.source_contract().get(),
            CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE,
            &args,
        );

        let caller = self.blockchain().get_caller();
        self.unclaimed_contracts(&caller).insert(contract_address.clone());

        let owner_address = self.blockchain().get_owner_address();
        self.send().direct_egld(&owner_address, &payment_amount);

        contract_address
    }

    #[endpoint(claimOwnership)]
    fn claim_ownership(&self, contract_address: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        require!(
            self.unclaimed_contracts(&caller).contains(&contract_address),
            "No contract(s) to claim"
        );

        self.unclaimed_contracts(&caller).swap_remove(&contract_address.clone());
        _ = self.send().change_owner_address(contract_address, &caller).async_call().call_and_exit();
    }
    
    #[only_owner]
    #[endpoint(setSourceContract)]
    fn set_source_contract(&self, contract: ManagedAddress) {
       self.source_contract().set(contract);
    }
    
    #[only_owner]
    #[endpoint(setContractPrice)]
    fn set_contract_price(&self, price: BigUint) {
        self.contract_price().set(price);
    }
}
