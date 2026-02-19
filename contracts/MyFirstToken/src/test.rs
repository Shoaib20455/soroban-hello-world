#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

#[test]
fn test_reward_flow() {
    let env = Env::default();
    env.mock_all_auths();

    // 1. Addresses setup karein
    let admin = Address::generate(&env);
    let player = Address::generate(&env);

    // 2. Pehle aik Nakli Token Contract (MFTK ki tarah) register karein
    // Taake hum Reward contract ko bata saken ke kis token mein reward dena hai
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token_client = token::Client::new(&env, &token_id);

    // 3. Reward Contract register karein
    let reward_contract_id = env.register(GameRewardContract, ());
    let reward_client = GameRewardContractClient::new(&env, &reward_contract_id);

    // 4. Initialize karein
    reward_client.initialize(&admin, &token_id);

    // 5. Reward Contract ke wallet mein kuch tokens bhejien (taake wo aage baant sakay)
    // Real life mein aap manually MFTK bhejenge, test mein hum mint kar rahe hain
    token_client.mint(&reward_contract_id, &1000);

    // 6. Game reward set karein (Game ID: 1, Amount: 50)
    reward_client.add_game_type(&1, &50);

    // 7. Reward distribute karein
    reward_client.distribute_reward(&player, &1);

    // 8. Verify karein ke player ko 50 tokens mil gaye
    assert_eq!(token_client.balance(&player), 50);
    // Aur contract ke paas 950 bache
    assert_eq!(token_client.balance(&reward_contract_id), 950);
}

#[test]
#[should_panic(expected = "AlreadyInitialized")]
fn test_cannot_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_id = Address::generate(&env);
    
    let reward_contract_id = env.register(GameRewardContract, ());
    let reward_client = GameRewardContractClient::new(&env, &reward_contract_id);

    reward_client.initialize(&admin, &token_id);
    // Dobara initialize karne par panic hona chahiye
    reward_client.initialize(&admin, &token_id);
}