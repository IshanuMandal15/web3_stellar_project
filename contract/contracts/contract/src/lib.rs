#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Lease {
    pub owner: Address,
    pub lessee: Option<Address>,
    pub price: i128,
    pub is_leased: bool,
}

#[contract]
pub struct AssetLeasingContract;

#[contractimpl]
impl AssetLeasingContract {

    // Create a new asset lease listing
    pub fn create_lease(env: Env, owner: Address, price: i128) {
        owner.require_auth();

        let lease = Lease {
            owner: owner.clone(),
            lessee: None,
            price,
            is_leased: false,
        };

        env.storage().instance().set(&Symbol::new(&env, "LEASE"), &lease);
    }

    // Lease the asset
    pub fn lease_asset(env: Env, lessee: Address) {
        lessee.require_auth();

        let key = Symbol::new(&env, "LEASE");
        let mut lease: Lease = env.storage().instance().get(&key).unwrap();

        if lease.is_leased {
            panic!("Asset already leased");
        }

        lease.lessee = Some(lessee);
        lease.is_leased = true;

        env.storage().instance().set(&key, &lease);
    }

    // Get lease info
    pub fn get_lease(env: Env) -> Lease {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "LEASE"))
            .unwrap()
    }
}