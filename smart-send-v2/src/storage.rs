use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("wegldIdentifier")]
    fn wegld_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("wegldContract")]
    fn wegld_contract(&self) -> SingleValueMapper<ManagedAddress>;
}
