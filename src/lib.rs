#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub enum DataKey {
    Deposit(u64),
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct DepositDetails {
    pub student: Address,
    pub landlord: Address,
    pub token: Address,
    pub amount: i128,
    pub is_refunded: bool,
}

#[contract]
pub struct DormDepositTrustContract;

#[contractimpl]
impl DormDepositTrustContract {
    /// Lock the student's deposit money safely in the contract
    pub fn create_deposit(
        env: Env,
        deposit_id: u64,
        student: Address,
        landlord: Address,
        token: Address,
        amount: i128,
    ) {
        student.require_auth();
        let client = token::Client::new(&env, &token);
        client.transfer(&student, &env.current_contract_address(), &amount);

        let details = DepositDetails {
            student,
            landlord,
            token,
            amount,
            is_refunded: false,
        };
        env.storage().persistent().set(&DataKey::Deposit(deposit_id), &details);
    }

    /// Send the deposit money back to the student after the landlord approves
    pub fn refund_deposit(env: Env, deposit_id: u64, landlord: Address) {
        landlord.require_auth();
        let mut details: DepositDetails = env
            .storage()
            .persistent()
            .get(&DataKey::Deposit(deposit_id))
            .expect("Deposit not found");

        if details.landlord != landlord {
            panic!("Unauthorized landlord");
        }
        if details.is_refunded {
            panic!("Deposit already refunded");
        }

        let client = token::Client::new(&env, &details.token);
        client.transfer(&env.current_contract_address(), &details.student, &details.amount);

        details.is_refunded = true;
        env.storage().persistent().set(&DataKey::Deposit(deposit_id), &details);
    }

    /// Read the saved deposit information
    pub fn get_deposit(env: Env, deposit_id: u64) -> DepositDetails {
        env.storage().persistent().get(&DataKey::Deposit(deposit_id)).expect("Deposit not found")
    }
}