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

/// ESPECIALLY LEAVE THESE TESTS ALONE
#[test]
fn fca00c_fast() {
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

    // We reset the budget to unlimited to asses the raw performance of the solution without hitting the TrapMemLimitExceeded or TrapCpuLimitExceeded error
    env.budget().reset_unlimited();

    solution.solve(&engine_id);

    let points = engine.p_points();

    println!("Points: {}", points);
    assert!(points >= 100);
}

#[test]
pub fn fca00c_budget() {
    let env = Env::default();
    let engine_id = env.register_contract_wasm(None, GameEngineWASM);
    let engine = GameEngine::new(&env, &engine_id);

    // DON'T CHANGE THE FOLLOWING INIT() PARAMETERS
    // Please see note in the `fca00c_fast()` function for more details.
    engine.init(&1, &3, &8891, &16, &(50, 5, 2, 1), &1, &6, &2);

    // We are running this test against your *compiled* solution contract,
    // rather than using your source code as a crate, like in `fca00c_fast()`.
    // The advantage here is: Your final submission will be a compiled wasm, and
    // this test will give you a better idea of what your final budget will be.
    // The drawback here is: Compiling your contract after each change will slow
    // you down, and make iterating more of a slog. This probably isn't the test
    // you want to _actively_ build against, but it is useful for fine-tuning a
    // valid contract.
    mod solution {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32-unknown-unknown/release/soroban_asteroids_solution.wasm"
        );
    }

    let solution_id = env.register_contract_wasm(None, solution::WASM);
    let solution = solution::Client::new(&env, &solution_id);

    // We reset the budget here so that we *only* count the budget for your
    // contract's `solve()` function. Everything else we've done so far is free!
    env.budget().reset_unlimited();

    solution.solve(&engine_id);

    // We are printing your contract's utilized budget, but there will be an
    // *expected* difference between the budget numbers you see locally and on
    // the FCA00C site. Please see above note for details.
    env.budget().print();

    let points = engine.p_points();

    println!("Points: {}", points);
    assert!(points >= 100);
}

// WRITE ANY OF YOUR OWN TESTS BELOW
