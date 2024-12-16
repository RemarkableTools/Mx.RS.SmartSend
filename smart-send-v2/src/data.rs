use multiversx_sc::imports::*;
use multiversx_sc::{
    api::ManagedTypeApi,
    codec,
    derive::{type_abi},
    proxy_imports::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{ManagedAddress},
};

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