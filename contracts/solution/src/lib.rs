#![no_std]

use engine::Client as GameEngine;
use soroban_sdk::{contractimpl, BytesN, Env};

pub struct Solution;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

mod test;

#[contractimpl]
impl Solution {
    pub fn solve(env: Env, engine_id: BytesN<32>) {
        let engine = GameEngine::new(&env, &engine_id);

        // YOUR CODE START

        // YOUR CODE END
    }
}
