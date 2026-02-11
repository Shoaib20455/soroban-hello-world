#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_token_initialization_and_mint() {
    let env = Env::default();
    env.mock_all_auths(); // Authorization checks ko simulate karne ke liye

    // 1. Test Addresses generate karein
    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);

    // 2. Contract register karein (__constructor ke arguments pass karein)
    // Register command format: env.register(Struct, (args, ...))
    let contract_id = env.register(MyFirstToken, (&recipient, &admin));
    let client = MyFirstTokenClient::new(&env, &contract_id);

    // 3. Metadata verify karein
    assert_eq!(client.name(), String::from_str(&env, "MyFirstToken"));
    assert_eq!(client.symbol(), String::from_str(&env, "MFTK"));
    assert_eq!(client.decimals(), 7);

    // 4. Initial Mint verify karein (10^16 units)
    assert_eq!(client.balance(&recipient), 10000000000000000);

    // 5. Owner verify karein
    assert_eq!(client.owner(), admin);

    // 6. Admin ke zariye mazeed minting test karein
    let mint_amount = 5000;
    client.mint(&recipient, &mint_amount);
    
    // Total balance check karein (Initial + New Mint)
    assert_eq!(client.balance(&recipient), 10000000000000000 + mint_amount);
}

#[test]
#[should_panic] // Ye test fail hona chahiye agar non-owner mint kare
fn test_non_owner_cannot_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let hacker = Address::generate(&env);
    let recipient = Address::generate(&env);

    let contract_id = env.register(MyFirstToken, (&recipient, &admin));
    let client = MyFirstTokenClient::new(&env, &contract_id);

    // Hacker mint karne ki koshish karega (Hacker is not the owner)
    // Ye line panic karegi kyunke #[only_owner] laga hua hai
    client.mint(&recipient, &1000);
}