#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Firm,        // The managing Law Firm
    PartyA,      // Attorney for Side A
    PartyB,      // Attorney for Side B
    Receiver,    // Final recipient of funds
    Asset,       // USDC/Asset address
    Amount,      // Total escrow amount
    ApprA,       // Approval status from Party A
    ApprB,       // Approval status from Party B
}

#[contract]
pub struct JurisTrustContract;

#[contractimpl]
impl JurisTrustContract {
    // Initialize the escrow with the legal parties and the amount to be held
    pub fn initialize(env: Env, firm: Address, a: Address, b: Address, receiver: Address, asset: Address, amount: i128) {
        if env.storage().instance().has(&DataKey::Firm) { panic!("Already initialized"); }
        
        env.storage().instance().set(&DataKey::Firm, &firm);
        env.storage().instance().set(&DataKey::PartyA, &a);
        env.storage().instance().set(&DataKey::PartyB, &b);
        env.storage().instance().set(&DataKey::Receiver, &receiver);
        env.storage().instance().set(&DataKey::Asset, &asset);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage().instance().set(&DataKey::ApprA, &false);
        env.storage().instance().set(&DataKey::ApprB, &false);
    }

    // Party A signs off on the release
    pub fn approve_a(env: Env, caller: Address) {
        let party_a: Address = env.storage().instance().get(&DataKey::PartyA).unwrap();
        if caller != party_a { panic!("Unauthorized"); }
        caller.require_auth();
        env.storage().instance().set(&DataKey::ApprA, &true);
        self::check_and_release(env);
    }

    // Party B signs off on the release
    pub fn approve_b(env: Env, caller: Address) {
        let party_b: Address = env.storage().instance().get(&DataKey::PartyB).unwrap();
        if caller != party_b { panic!("Unauthorized"); }
        caller.require_auth();
        env.storage().instance().set(&DataKey::ApprB, &true);
        self::check_and_release(env);
    }
}

// Internal function to check if both parties have signed and release funds
fn check_and_release(env: Env) {
    let a: bool = env.storage().instance().get(&DataKey::ApprA).unwrap();
    let b: bool = env.storage().instance().get(&DataKey::ApprB).unwrap();

    if a && b {
        let asset_addr: Address = env.storage().instance().get(&DataKey::Asset).unwrap();
        let receiver: Address = env.storage().instance().get(&DataKey::Receiver).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();
        
        let client = token::Client::new(&env, &asset_addr);
        client.transfer(&env.current_contract_address(), &receiver, &amount);
    }
}