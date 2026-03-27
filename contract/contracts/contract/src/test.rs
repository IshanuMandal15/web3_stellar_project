#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_asset_leasing_flow() {
    let env = Env::default();

    let contract_id = env.register_contract(None, AssetLeasingContract);
    let client = AssetLeasingContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let lessee = Address::generate(&env);

    // Create lease
    client.create_lease(&owner, &1000);

    let lease = client.get_lease();
    assert_eq!(lease.price, 1000);
    assert_eq!(lease.is_leased, false);

    // Lease asset
    client.lease_asset(&lessee);

    let updated = client.get_lease();
    assert_eq!(updated.is_leased, true);
    assert!(updated.lessee.is_some());
}