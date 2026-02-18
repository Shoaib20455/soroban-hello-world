#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    panic_with_error, token, Address, Env, Vec,
};

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum RewardError {
    AlreadyInitialized = 1,
    NotAdmin = 2,
    GameNotFound = 3,
    InvalidToken = 4,
    InvalidAmount = 5,
    AlreadyClaimed = 6,
}

#[contracttype]
pub enum DataKey {
    Admin,
    TokenID,
    GameReward(u32),
    GameClaimed(u32, Address),
    PlayerGames(Address),
}

#[contract]
pub struct GameRewardContract;

#[contractimpl]
impl GameRewardContract {

    pub fn initialize(env: Env, admin: Address, token_id: Address) {
    if env.storage().instance().has(&DataKey::Admin) {
        panic_with_error!(&env, RewardError::AlreadyInitialized);
    }

    admin.require_auth();

    let token_client = token::Client::new(&env, &token_id);
    
    // decimals() returns u32, so check if 0 (invalid token)
    let decimals: u32 = token_client.decimals();
    if decimals == 0 {
        panic_with_error!(&env, RewardError::InvalidToken);
    }

    env.storage().instance().set(&DataKey::Admin, &admin);
    env.storage().instance().set(&DataKey::TokenID, &token_id);
}


    pub fn add_game_type(env: Env, game_id: u32, reward_amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        if reward_amount <= 0 {
            panic_with_error!(&env, RewardError::InvalidAmount);
        }

        env.storage()
            .persistent()
            .set(&DataKey::GameReward(game_id), &reward_amount);
    }

    pub fn distribute_reward(env: Env, to: Address, game_id: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let reward_key = DataKey::GameReward(game_id);
        if !env.storage().persistent().has(&reward_key) {
            panic_with_error!(&env, RewardError::GameNotFound);
        }

        // Replay Protection
        let claim_key = DataKey::GameClaimed(game_id, to.clone());
        if env.storage().persistent().has(&claim_key) {
            panic_with_error!(&env, RewardError::AlreadyClaimed);
        }

        let amount: i128 = env.storage().persistent().get(&reward_key).unwrap();

        let token_id: Address = env.storage().instance().get(&DataKey::TokenID).unwrap();
        let token_client = token::Client::new(&env, &token_id);
        let contract_address = env.current_contract_address();

        token_client.transfer(&contract_address, &to, &amount);

        // mark as claimed
        env.storage().persistent().set(&claim_key, &true);

        // 🔵 maintain player history
        let history_key = DataKey::PlayerGames(to.clone());

        let mut games: Vec<u32> =
            env.storage().persistent().get(&history_key).unwrap_or(Vec::new(&env));

        games.push_back(game_id);

        env.storage().persistent().set(&history_key, &games);

        env.events().publish((game_id, "reward_distributed"), to);
    }

    pub fn get_player_status(env: Env, player: Address, game_id: u32) -> bool {
    let claim_key = DataKey::GameClaimed(game_id, player);

    if env.storage().persistent().has(&claim_key) {
        true
    } else {
        false
    }
}



    pub fn withdraw(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, RewardError::InvalidAmount);
        }

        let token_id: Address = env.storage().instance().get(&DataKey::TokenID).unwrap();
        let token_client = token::Client::new(&env, &token_id);
        let contract_address = env.current_contract_address();

        token_client.transfer(&contract_address, &to, &amount);

        env.events().publish(("withdraw",), (to.clone(), amount));
    }

    pub fn upgrade(env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
