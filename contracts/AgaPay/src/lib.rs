#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env,
};

#[contract]
pub struct BayanihanReliefContract;

#[contracttype]
#[derive(Clone)]
pub struct Voucher {
    pub recipient: Address,
    pub amount: i128,
    pub redeemed: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Voucher(u32),
    VoucherCount,
}

#[contractimpl]
impl BayanihanReliefContract {

    // Initialize the contract with admin
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::VoucherCount, &0u32);
    }

    // Issue disaster relief voucher
    pub fn issue_voucher(
        env: Env,
        admin: Address,
        recipient: Address,
        amount: i128,
    ) -> u32 {

        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("Unauthorized admin");
        }

        let mut count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::VoucherCount)
            .unwrap();

        count += 1;

        let voucher = Voucher {
            recipient,
            amount,
            redeemed: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Voucher(count), &voucher);

        env.storage()
            .instance()
            .set(&DataKey::VoucherCount, &count);

        count
    }

    // Redeem voucher
    pub fn redeem_voucher(
        env: Env,
        recipient: Address,
        voucher_id: u32,
    ) {

        recipient.require_auth();

        let key = DataKey::Voucher(voucher_id);

        let mut voucher: Voucher = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if voucher.recipient != recipient {
            panic!("Not voucher owner");
        }

        if voucher.redeemed {
            panic!("Voucher already redeemed");
        }

        voucher.redeemed = true;

        env.storage()
            .persistent()
            .set(&key, &voucher);
    }

    // Get voucher info
    pub fn get_voucher(env: Env, voucher_id: u32) -> Voucher {
        env.storage()
            .persistent()
            .get(&DataKey::Voucher(voucher_id))
            .unwrap()
    }

    // Get total vouchers issued
    pub fn total_vouchers(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::VoucherCount)
            .unwrap_or(0)
    }
}