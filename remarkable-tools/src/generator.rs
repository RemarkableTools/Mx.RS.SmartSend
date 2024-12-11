use multiversx_sc::imports::*;
use core::ops::Deref;
use proxies::smart_send_proxy::SmartSendProxy;

#[multiversx_sc::module]
pub trait GeneratorModule {
    #[payable("*")]
    #[endpoint(generate)]
    fn generate(&self) -> ManagedAddress {
        let egld_value = self.call_value().egld_value();
        let payment_amount = egld_value.deref();
        require!(
            payment_amount == &self.contract_price().get(),
            "Invalid EGLD payment amount"
        );

        let contract_address = self.tx()
            .typed(SmartSendProxy)
            .init()
            .from_source(self.source_contract().get())
            .code_metadata(CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE)
            .returns(ReturnsNewManagedAddress)
            .sync_call();

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

        self.tx()
            .to(contract_address.clone())
            .typed(UserBuiltinProxy)
            .change_owner_address(&caller)
            .sync_call();
        self.unclaimed_contracts(&caller).swap_remove(&contract_address);
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

    //----------------------------------------------------------------------------------------------
    #[view(getSourceContract)]
    #[storage_mapper("sourceContract")]
    fn source_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getGenerationPrice)]
    #[storage_mapper("generationPrice")]
    fn contract_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getUnclaimedContracts)]
    #[storage_mapper("unclaimedContracts")]
    fn unclaimed_contracts(&self, address: &ManagedAddress) -> UnorderedSetMapper<ManagedAddress>;
}
