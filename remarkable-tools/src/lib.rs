#![no_std]

use multiversx_sc::imports::*;

mod transfer;
mod generator;

#[multiversx_sc::contract]
pub trait RemarkableToolsSmartSend: generator::GeneratorModule + transfer::TransferModule + distribution::DistributionModule {
    #[init]
    fn init(&self, smart_send_source: ManagedAddress) {
        self.contract_price().set(BigUint::zero());
        self.source_contract().set(smart_send_source);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
