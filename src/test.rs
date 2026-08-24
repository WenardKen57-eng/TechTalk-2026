#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn create_token_contract<'a>(e: &'a Env, admin: &'a Address) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let contract_id = e.register_stellar_asset_contract(admin.clone());
    (token::Client::new(e, &contract_id), token::StellarAssetClient::new(e, &contract_id))
}

#[test]
fn test_1_happy_path_landlord_refund() {
    let env = Env::default();
    env.mock_all_authorizations();

    let contract_id = env.register_contract(None, DormDepositTrustContract);
    let client = DormDepositTrustContractClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let student = Address::generate(&env);
    let landlord = Address::generate(&env);

    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&student, &100);

    client.create_deposit(&1, &student, &landlord, &token_client.address, &50);
    client.refund_deposit(&1, &landlord);

    assert_eq!(token_client.balance(&student), 100);
}

#[test]
#[should_panic(expected = "Unauthorized landlord")]
fn test_2_unauthorized_caller_fails() {
    let env = Env::default();
    env.mock_all_authorizations();

    let contract_id = env.register_contract(None, DormDepositTrustContract);
    let client = DormDepositTrustContractClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let student = Address::generate(&env);
    let landlord = Address::generate(&env);
    let stranger = Address::generate(&env);

    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&student, &100);

    client.create_deposit(&1, &student, &landlord, &token_client.address, &50);
    client.refund_deposit(&1, &stranger);
}

#[test]
#[should_panic(expected = "Deposit already refunded")]
fn test_3_duplicate_refund_fails() {
    let env = Env::default();
    env.mock_all_authorizations();

    let contract_id = env.register_contract(None, DormDepositTrustContract);
    let client = DormDepositTrustContractClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let student = Address::generate(&env);
    let landlord = Address::generate(&env);

    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&student, &100);

    client.create_deposit(&1, &student, &landlord, &token_client.address, &50);
    client.refund_deposit(&1, &landlord);
    client.refund_deposit(&1, &landlord);
}

#[test]
#[should_panic(expected = "Deposit not found")]
fn test_4_invalid_deposit_id_fails() {
    let env = Env::default();
    env.mock_all_authorizations();

    let contract_id = env.register_contract(None, DormDepositTrustContract);
    let client = DormDepositTrustContractClient::new(&env, &contract_id);
    let landlord = Address::generate(&env);

    client.refund_deposit(&999, &landlord);
}

#[test]
fn test_5_state_verification() {
    let env = Env::default();
    env.mock_all_authorizations();

    let contract_id = env.register_contract(None, DormDepositTrustContract);
    let client = DormDepositTrustContractClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let student = Address::generate(&env);
    let landlord = Address::generate(&env);

    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&student, &100);

    client.create_deposit(&1, &student, &landlord, &token_client.address, &50);
    client.refund_deposit(&1, &landlord);

    let deposit = client.get_deposit(&1);
    assert_eq!(deposit.is_refunded, true);
}