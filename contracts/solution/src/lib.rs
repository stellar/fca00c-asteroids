#![no_std]

use engine::{Client as G, Direction as D, MapElement as M};
use soroban_sdk::{contractimpl, BytesN, Env};

pub struct Solution;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

mod test;

#[contractimpl]
impl Solution {
    pub fn solve(env: Env, eid: BytesN<32>) {
        let e = G::new(&env, &eid);

        // YOUR CODE START
        while e.p_points() < 100 {
            for (pos, ele) in e.get_map().iter().filter_map(Result::ok) {
                let x = pos.0 - e.p_pos().0;
                let mut dx = D::Right;
                let y = pos.1 - e.p_pos().1;
                let mut dy = D::Up;
                if x < 0 {
                    dx = D::Left;
                }
                e.p_turn(&dx);
                e.p_move(&Some(x.unsigned_abs()));
                if y < 0 {
                    dy = D::Down;
                }
                e.p_turn(&dy);
                e.p_move(&Some(y.unsigned_abs()));
                match ele {
                    M::FuelPod => e.p_harvest(),
                    M::Asteroid => e.p_shoot(),
                }
            }
            if e.get_map().is_empty() {
                e.p_turn(&D::UpRight);
                e.p_move(&Some(1));
            }
        }
        
        // YOUR CODE END
    }
}
