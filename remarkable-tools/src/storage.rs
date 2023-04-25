multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
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