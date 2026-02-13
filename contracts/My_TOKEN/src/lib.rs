// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.6.0
#![no_std]

use soroban_sdk::{
    Address, contract, contractimpl, Env, MuxedAddress, String, Symbol, Vec
};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_macros::only_owner;
use stellar_tokens::fungible::{Base, burnable::FungibleBurnable, FungibleToken};

#[contract]
pub struct MyFirstToken;

#[contractimpl]
impl MyFirstToken {
    pub fn __constructor(e: &Env, recipient: Address, owner: Address) {
        Base::set_metadata(e, 7, String::from_str(e, "MyFirstToken"), String::from_str(e, "MFTK"));
        Base::mint(e, &recipient, 10000000000000000);
        ownable::set_owner(e, &owner);
    }

    #[only_owner]
    pub fn mint(e: &Env, account: Address, amount: i128) {
        Base::mint(e, &account, amount);
    }
}

#[contractimpl(contracttrait)]
impl FungibleToken for MyFirstToken {
    type ContractType = Base;

}

//
// Extensions
//

#[contractimpl(contracttrait)]
impl FungibleBurnable for MyFirstToken {}

//
// Utils
//

#[contractimpl(contracttrait)]
impl Ownable for MyFirstToken {}
