#![cfg(test)]
//! Regression tests for the prebuilt `contracts/game_engine.wasm`.
//!
//! These run against the committed WASM (not the native crate) because the map
//! generator uses `rand::rngs::SmallRng`, whose algorithm differs between
//! 32-bit (wasm) and 64-bit (native) targets. The expected layout below was
//! captured from the original soroban-sdk 0.8 build of the engine, so a passing
//! test means the rebuilt engine still produces the same galaxies.
extern crate std;

use soroban_sdk::Env;
use std::vec::Vec;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

use engine::{Client as GameEngine, Direction, MapElement, WASM};

fn layout(engine: &GameEngine) -> Vec<(i32, i32, bool)> {
    engine
        .get_map()
        .iter()
        .map(|(p, el)| (p.0, p.1, el == MapElement::Asteroid))
        .collect()
}

#[test]
fn galaxy_layout_is_stable() {
    let env = Env::default();
    let engine_id = env.register(WASM, ());
    let engine = GameEngine::new(&env, &engine_id);
    engine.init(&1, &3, &8891, &16, &(50, 5, 2, 1), &1, &6, &2);

    // (x, y, is_asteroid); `false` is a fuel pod.
    assert_eq!(
        layout(&engine),
        [
            (2, 4, true),
            (4, 7, false),
            (8, 10, true),
            (10, 14, true),
            (14, 1, false),
            (14, 7, true),
            (15, 11, true),
            (16, 0, true),
        ]
    );

    engine.p_turn(&Direction::Right);
    engine.p_move(&Some(17));
    assert_eq!(engine.p_pos(), engine::Point(25, 8));
    assert_eq!(
        layout(&engine),
        [
            (18, 4, true),
            (19, 9, false),
            (21, 8, false),
            (22, 7, true),
            (22, 9, true),
            (28, 8, true),
            (30, 10, true),
            (33, 12, true),
        ]
    );
    assert_eq!(engine.p_fuel(), 15);
}

#[test]
fn shoot_harvest_and_fuel() {
    let env = Env::default();
    let engine_id = env.register(WASM, ());
    let engine = GameEngine::new(&env, &engine_id);
    engine.init(&1, &3, &8891, &16, &(50, 5, 2, 1), &1, &6, &2);

    // From (8, 8) facing Up, the asteroid at (8, 10) is within laser range 3.
    engine.p_shoot();
    assert_eq!(engine.p_points(), 1);
    assert_eq!(engine.p_fuel(), 45);
    assert!(!layout(&engine).contains(&(8, 10, true)));

    // Re-initializing is rejected.
    assert!(engine
        .try_init(&1, &3, &8891, &16, &(50, 5, 2, 1), &1, &6, &2)
        .is_err());

    // Running out of fuel returns NotEnoughFuel.
    assert_eq!(
        engine.try_p_move(&Some(100)),
        Err(Ok(engine::Error::NotEnoughFuel))
    );
}
