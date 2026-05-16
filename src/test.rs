#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, Env,
};

fn setup() -> (Env, SimpleWhitelistClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(SimpleWhitelist, ());
    let client = SimpleWhitelistClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    client.initialize(&owner);
    (env, client, owner)
}

// --- initialize ---

#[test]
fn test_initialize_sets_owner() {
    let (env, client, owner) = setup();
    let stored: Address = env
        .as_contract(&client.address, || {
            env.storage().instance().get(&DataKey::Owner).unwrap()
        });
    assert_eq!(stored, owner);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice_panics() {
    let (_, client, owner) = setup();
    client.initialize(&owner);
}

// --- add_address ---

#[test]
fn test_add_address() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.add_address(&user);
    assert!(client.is_whitelisted(&user));
}

#[test]
fn test_add_address_duplicate_is_idempotent() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.add_address(&user);
    client.add_address(&user);
    assert!(client.is_whitelisted(&user));
}

// --- remove_address ---

#[test]
fn test_remove_address() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.add_address(&user);
    client.remove_address(&user);
    assert!(!client.is_whitelisted(&user));
}

#[test]
fn test_remove_nonexistent_is_idempotent() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.remove_address(&user);
    assert!(!client.is_whitelisted(&user));
}

// --- is_whitelisted ---

#[test]
fn test_is_whitelisted_false_for_unknown() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    assert!(!client.is_whitelisted(&user));
}

// --- events ---

#[test]
fn test_add_emits_whitelisted_event() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.add_address(&user);
    // At least one event must have been emitted
    assert!(!env.events().all().is_empty());
}

#[test]
fn test_remove_emits_removed_event() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.add_address(&user);
    client.remove_address(&user);
    // The remove call must have emitted at least one event
    assert!(!env.events().all().is_empty());
}
