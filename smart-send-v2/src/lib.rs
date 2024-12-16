#![no_std]

use multiversx_sc::imports::*;

mod transfer;
mod users;
mod storage;
mod data;

#[multiversx_sc::contract]
pub trait SmartSendV2:
    transfer::TransferModule
    + distribution::DistributionModule
    + users::UsersModule
    + storage::StorageModule
{
    #[init]
    fn init(
        &self,
        wegld_identifier: TokenIdentifier,
    ) {
        self.wegld_identifier().set(wegld_identifier);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
