#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _},
    Address, Env,
};

use crate::{BayanihanReliefContract, BayanihanReliefContractClient};

#[test]
fn test_initialize_contract() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanihanReliefContract);

    let client = BayanihanReliefContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin);

    let total = client.total_vouchers();

    assert_eq!(total, 0);
}

#[test]
fn test_issue_voucher() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanihanReliefContract);

    let client = BayanihanReliefContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.initialize(&admin);

    let voucher_id = client.issue_voucher(
        &admin,
        &recipient,
        &500i128,
    );

    assert_eq!(voucher_id, 1);

    let voucher = client.get_voucher(&1);

    assert_eq!(voucher.amount, 500);
    assert_eq!(voucher.redeemed, false);
}

#[test]
fn test_redeem_voucher() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanihanReliefContract);

    let client = BayanihanReliefContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.initialize(&admin);

    client.issue_voucher(
        &admin,
        &recipient,
        &1000i128,
    );

    client.redeem_voucher(
        &recipient,
        &1,
    );

    let voucher = client.get_voucher(&1);

    assert_eq!(voucher.redeemed, true);
}

#[test]
#[should_panic(expected = "Voucher already redeemed")]
fn test_double_redeem_should_fail() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanihanReliefContract);

    let client = BayanihanReliefContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.initialize(&admin);

    client.issue_voucher(
        &admin,
        &recipient,
        &700i128,
    );

    client.redeem_voucher(
        &recipient,
        &1,
    );

    client.redeem_voucher(
        &recipient,
        &1,
    );
}

#[test]
fn test_total_vouchers() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanihanReliefContract);

    let client = BayanihanReliefContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin);

    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);

    client.issue_voucher(
        &admin,
        &recipient1,
        &500i128,
    );

    client.issue_voucher(
        &admin,
        &recipient2,
        &1000i128,
    );

    let total = client.total_vouchers();

    assert_eq!(total, 2);
}