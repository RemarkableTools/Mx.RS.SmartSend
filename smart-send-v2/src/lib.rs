#![no_std]

use multiversx_sc::imports::*;

mod transfer;
mod users;

#[multiversx_sc::contract]
pub trait SmartSendV2:
    transfer::TransferModule
    + distribution::DistributionModule
    + users::UsersModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
