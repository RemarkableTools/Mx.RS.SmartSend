use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait UsersModule {
    #[only_owner]
    #[endpoint(addUser)]
    fn add_user(&self, user: ManagedAddress) {
        self.allowed_users().insert(user);
    }

    #[only_owner]
    #[endpoint(removeUser)]
    fn remove_user(&self, user: ManagedAddress) {
        self.allowed_users().swap_remove(&user);
    }

    #[inline]
    fn require_user_is_allowed(&self, user: ManagedAddress) {
        require!(
            self.blockchain().get_owner_address() == user || self.allowed_users().contains(&user),
            "Caller is not allowed to use the contract"
        );
    }

    //----------------------------------------------------------------------------------------------
    #[view(getAllowedUsers)]
    #[storage_mapper("allowedUsers")]
    fn allowed_users(&self) -> UnorderedSetMapper<ManagedAddress>;
}
