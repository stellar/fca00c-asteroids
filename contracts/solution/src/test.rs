#![cfg(test)]
/// THERE IS NO NEED TO EDIT THIS FILE
/// This test file closely mimics how we will evaluate your submitted contract.
/// You are free to write your own tests during development, but please be aware
/// that the `test()` function below lays out some parameters and patterns
/// you'll want to use in your test setup.

/// A Note on Budget: The following `test()` function will calculate and display
/// the cost of running your contract, in terms of CPU instructions and memory
/// bytes. There is an *expected* difference between results seen when testing
/// locally, and results given when submitted for evaluation on the FCA00C site.
/// You can expect your numbers locally to be lower than your "official" cost
/// when your contract is submitted for evaluation. However, as you optimize
/// your contract and lower your cost locally, you can expect your "official"
/// cost will decrease as well.
use std::println;

use soroban_sdk::Env;

use crate::{
    engine::{Client as GameEngine, WASM as GameEngineWASM},
    Solution, SolutionClient,
};

extern crate std;

/// ESPECIALLY LEAVE THIS TEST ALONE
#[test]
fn test() {
    // Here we install and register the GameEngine contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let env = Env::default();
    let engine_id = env.register_contract_wasm(None, GameEngineWASM);
    let engine = GameEngine::new(&env, &engine_id);

    // DON'T CHANGE THE FOLLOWING INIT() PARAMETERS
    // Once you've submitted your contract on the FCA00C site, we will invoke
    // the `init()` function of our GameEngine contract with the same values
    // every time. These will be the *EXACT SAME VALUES* for each and every
    // submission, from each and every player. Leaving them as-is in this test
    // will allow you to locally simulate (as closely as possible) the results
    // you'll see when you really submit your contract
    engine.init(
        &1,    // The number of spaces your ship will `p_move()` by default
        &3,    // The maximum distance from which your ship's laser can `p_shoot()` an asteroid
        &8891, // The map's randomness is seeded with a known, consistent `u64` value (this will produce the same map every time for everybody)
        &16,   // The size of each galaxy grid (so 16x16)
        &(
            // Soroban functions can only have a maximum of 10 parameters, so all the fuel parameters are collected here
            50, // The amount of fuel your ship contains at initialization
            5,  // The amount of fuel consumed by the `p_shoot()` method
            2,  // The amount of fuel consumed when you `p_move()` a single space
            1,  // The amount of fuel consumed by the `p_turn()` method
        ),
        &1, // The number of points you are rewarded for destroying an asteroid
        &6, // The number of asteroids each galaxy will contain
        &2, // The number of fuel pods each galaxy will contain
    );

    let solution_id = env.register_contract(None, Solution);
    let solution = SolutionClient::new(&env, &solution_id);

    env.budget().reset();

    solution.solve(&engine_id);

    // We are printing your contract's utilized budget, but there will be an
    // *expected* difference between the budget numbers you see locally and on
    // the FCA00C site. Please see above note for details.
    env.budget().print();

    let points = engine.p_points();

    println!("Points: {}", points);
    assert!(engine.p_points() >= 100);
}

// WRITE ANY OF YOUR OWN TESTS BELOW
