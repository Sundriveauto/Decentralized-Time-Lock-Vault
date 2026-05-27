use soroban_sdk::{Address, Env, Symbol, symbol_short};

pub fn deposit(env: &Env, depositor: &Address, deposit_id: u32, token: &Address, amount: i128, unlock_time: u64) {
    let topics = (symbol_short!("deposit"), depositor.clone(), token.clone());
    env.events().publish(topics, (deposit_id, amount, unlock_time));
}

pub fn withdraw(env: &Env, depositor: &Address, deposit_id: u32, token: &Address, amount: i128) {
    let topics = (symbol_short!("withdraw"), depositor.clone(), token.clone());
    env.events().publish(topics, (deposit_id, amount));
}

pub fn emergency_withdraw(
    env: &Env,
    admin: &Address,
    depositor: &Address,
    deposit_id: u32,
    token: &Address,
    amount: i128,
) {
    let topics = (
        Symbol::new(env, "emrg_wdraw"),
        admin.clone(),
        depositor.clone(),
    );
    env.events().publish(topics, (deposit_id, token.clone(), amount));
}

pub fn admin_transfer_initiated(env: &Env, current_admin: &Address, pending_admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_init"), current_admin.clone());
    env.events().publish(topics, pending_admin.clone());
}

pub fn admin_transfer_accepted(env: &Env, new_admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_done"), new_admin.clone());
    env.events().publish(topics, ());
}

pub fn admin_renounced(env: &Env, former_admin: &Address) {
    let topics = (Symbol::new(env, "adm_renounce"), former_admin.clone());
    env.events().publish(topics, ());
}
