#![cfg(test)]

use crate::{ConcentratedLiquidityPool, ConcentratedLiquidityPoolClient};
use crate::{POSITION_BUMP_TO, POSITION_TTL_THRESHOLD};
use soroban_sdk::{testutils::{Ledger, LedgerInfo}, Env};

fn advance_ledger(env: &Env, by: u32) {
    let info = env.ledger().get();
    env.ledger().set(LedgerInfo {
        sequence_number: info.sequence_number + by,
        timestamp: info.timestamp + (by as u64) * 5,
        ..info
    });
}

#[test]
fn test_get_position_bumps_ttl_after_threshold() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, ConcentratedLiquidityPool);
    let client = ConcentratedLiquidityPoolClient::new(&env, &contract_id);

    let owner = soroban_sdk::Address::generate(&env);
    let position_id = client.open_position_for_test(&owner, &-100, &100, &1_000_000);

    advance_ledger(&env, POSITION_TTL_THRESHOLD + 1);

    let position = client.get_position(&position_id);
    assert_eq!(position.id, position_id);

    let ttl = client.position_ttl_for_test(&position_id);
    assert!(ttl >= POSITION_BUMP_TO);
}

#[test]
fn test_get_positions_bumps_ttl_after_threshold() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, ConcentratedLiquidityPool);
    let client = ConcentratedLiquidityPoolClient::new(&env, &contract_id);

    let owner = soroban_sdk::Address::generate(&env);
    let position_id = client.open_position_for_test(&owner, &-100, &100, &1_000_000);

    advance_ledger(&env, POSITION_TTL_THRESHOLD + 1);

    let positions = client.get_positions(&owner);
    assert_eq!(positions.len(), 1);
    assert_eq!(positions.get(0).unwrap().id, position_id);

    let ttl = client.position_list_ttl_for_test(&owner);
    assert!(ttl >= POSITION_BUMP_TO);
}

#[test]
fn test_check_range_order_filled_bumps_position_ttl() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, ConcentratedLiquidityPool);
    let client = ConcentratedLiquidityPoolClient::new(&env, &contract_id);

    let owner = soroban_sdk::Address::generate(&env);
    let position_id = client.open_range_order_for_test(&owner, &100, &1_000_000);

    advance_ledger(&env, POSITION_TTL_THRESHOLD + 1);

    let _ = client.check_range_order_filled(&position_id);

    let ttl = client.position_ttl_for_test(&position_id);
    assert!(ttl >= POSITION_BUMP_TO);
}
