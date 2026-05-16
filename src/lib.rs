#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol};

#[contracttype]
pub enum DataKey {
    Owner,
    Whitelist,
}

#[contract]
pub struct SimpleWhitelist;

#[contractimpl]
impl SimpleWhitelist {
    /// One-time initialization. Panics if already initialized.
    pub fn initialize(env: Env, owner: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage()
            .instance()
            .set(&DataKey::Whitelist, &Map::<Address, bool>::new(&env));
    }

    /// Add `user` to the whitelist. Owner auth required.
    pub fn add_address(env: Env, user: Address) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        let mut whitelist: Map<Address, bool> =
            env.storage().instance().get(&DataKey::Whitelist).unwrap();
        whitelist.set(user.clone(), true);
        env.storage().instance().set(&DataKey::Whitelist, &whitelist);

        env.events()
            .publish((Symbol::new(&env, "Whitelisted"),), (user,));
    }

    /// Remove `user` from the whitelist. Owner auth required.
    pub fn remove_address(env: Env, user: Address) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        let mut whitelist: Map<Address, bool> =
            env.storage().instance().get(&DataKey::Whitelist).unwrap();
        whitelist.remove(user.clone());
        env.storage().instance().set(&DataKey::Whitelist, &whitelist);

        env.events()
            .publish((Symbol::new(&env, "Removed"),), (user,));
    }

    /// Returns true if `user` is whitelisted.
    pub fn is_whitelisted(env: Env, user: Address) -> bool {
        let whitelist: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&DataKey::Whitelist)
            .unwrap_or(Map::new(&env));
        whitelist.get(user).unwrap_or(false)
    }
}

#[cfg(test)]
mod test;
