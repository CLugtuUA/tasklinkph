#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, BytesN};

#[test]
fn test_happy_path_ph_payout() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TaskLinkPH);
    let client = TaskLinkPHClient::new(&env, &contract_id);

    let client_addr = Address::generate(&env);
    let freelancer_addr = Address::generate(&env);
    let amount = 25000_0000000_i128; // Represents equivalent contract token funding
    let task_id = BytesN::from_array(&env, &[7u8; 32]);

    // 1. Client creates milestone task link
    client.create_milestone(&client_addr, &freelancer_addr, &amount, &task_id);

    // 2. Client signs off and funds route to freelancer
    let state = client.complete_and_release(&task_id);
    assert_eq!(state.status, MilestoneStatus::Completed);
}

#[test]
#[should_panic(expected = "Milestone is already finalized")]
fn test_edge_case_prevent_double_settlement() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TaskLinkPH);
    let client = TaskLinkPHClient::new(&env, &contract_id);

    let client_addr = Address::generate(&env);
    let freelancer_addr = Address::generate(&env);
    let amount = 15000_0000000_i128;
    let task_id = BytesN::from_array(&env, &[8u8; 32]);

    client.create_milestone(&client_addr, &freelancer_addr, &amount, &task_id);
    client.complete_and_release(&task_id);
    
    // This second call must fail execution flow
    client.complete_and_release(&task_id);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TaskLinkPH);
    let client = TaskLinkPHClient::new(&env, &contract_id);

    let client_addr = Address::generate(&env);
    let freelancer_addr = Address::generate(&env);
    let amount = 50000_0000000_i128;
    let task_id = BytesN::from_array(&env, &[9u8; 32]);

    let verified_state = client.create_milestone(&client_addr, &freelancer_addr, &amount, &task_id);
    assert_eq!(verified_state.status, MilestoneStatus::Funded);
    assert_eq!(verified_state.amount, 50000_0000000_i128);
}

#[test]
#[should_panic(expected = "Milestone amount must be positive")]
fn test_edge_case_zero_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TaskLinkPH);
    let client = TaskLinkPHClient::new(&env, &contract_id);

    let client_addr = Address::generate(&env);
    let freelancer_addr = Address::generate(&env);
    let amount = 0_i128;
    let task_id = BytesN::from_array(&env, &[10u8; 32]);

    client.create_milestone(&client_addr, &freelancer_addr, &amount, &task_id);
}

#[test]
#[should_panic(expected = "Task milestone registry not found")]
fn test_edge_case_missing_milestone() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TaskLinkPH);
    let client = TaskLinkPHClient::new(&env, &contract_id);

    let missing_id = BytesN::from_array(&env, &[99u8; 32]);
    client.complete_and_release(&missing_id);
}