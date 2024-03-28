#![allow(clippy::too_many_arguments)]

use crate::map::{build_range_map, calc_center, get_laser_collisions};
use crate::storage::{
    change_direction, change_position, decrement_fuel, decrement_points, get_direction, get_fuel,
    get_laser_range, get_move_fuel, get_points, get_position, get_reward_amount, get_shoot_fuel,
    get_step, get_turn_fuel, increment_fuel, increment_points, set_expired, set_move_fuel,
    set_shoot_fuel, set_turn_fuel,
};
use crate::types::{DataKey, Direction, Error, MapElement, Point};

use soroban_sdk::{contract, contractimpl, panic_with_error, Env, Map};

#[contract]
pub struct GameEngine;

#[contractimpl]
impl GameEngine {
    /// Initialize the engine contract.
    pub fn init(
        e: Env,
        move_step: u32,
        laser_range: u32,
        seed: u64,
        view_range: u32,
        fuel: (u32, u32, u32, u32),
        asteroid_reward: u32,
        asteroid_density: u32,
        pod_density: u32,
    ) {
        if e.storage().instance().has(&DataKey::Seed) {
            panic_with_error!(&e, Error::UnknownErr);
        }

        e.storage().instance().set(&DataKey::MoveStep, &move_step);
        e.storage()
            .instance()
            .set(&DataKey::LaserRange, &laser_range);
        e.storage().instance().set(&DataKey::Seed, &(seed + 2));
        e.storage().instance().set(&DataKey::Range, &view_range);
        e.storage()
            .instance()
            .set(&DataKey::Reward, &asteroid_reward);
        e.storage().instance().set(&DataKey::PlayerFuel, &fuel.0);
        e.storage()
            .instance()
            .set(&DataKey::PlayerPos, &Point(8, 8));
        e.storage()
            .instance()
            .set(&DataKey::PlayerDir, &Direction::Up);
        e.storage().instance().set(&DataKey::Points, &0_u32);
        set_shoot_fuel(&e, fuel.1);
        set_turn_fuel(&e, fuel.3);
        set_move_fuel(&e, fuel.2);
        e.storage()
            .instance()
            .set(&DataKey::AstDensity, &asteroid_density);
        e.storage()
            .instance()
            .set(&DataKey::PodDensity, &pod_density);
    }

    /// Turn player direction.
    pub fn p_turn(e: Env, direction: Direction) -> Result<(), Error> {
        change_direction(&e, direction);
        decrement_fuel(&e, get_turn_fuel(&e))
    }

    /// Move the player in the grid by `move_step`. An Option with the number of times to make the move can also be supplied, if not the engine will move the player once.
    pub fn p_move(e: Env, times: Option<u32>) -> Result<(), Error> {
        let direction: Direction = get_direction(&e);
        let player_pos: Point = get_position(&e);

        let step: i32;
        let mut used_fuel_mul: u32 = 1;

        if let Some(n) = times {
            step = (get_step(&e) * n) as i32;
            used_fuel_mul = n;
        } else {
            step = get_step(&e) as i32;
        }

        let point = match direction {
            Direction::Up => Point(player_pos.0, player_pos.1 + step),
            Direction::UpRight => Point(player_pos.0 + step, player_pos.1 + step),
            Direction::Right => Point(player_pos.0 + step, player_pos.1),
            Direction::DownRight => Point(player_pos.0 + step, player_pos.1 - step),
            Direction::Down => Point(player_pos.0, player_pos.1 - step),
            Direction::DownLeft => Point(player_pos.0 - step, player_pos.1 - step),
            Direction::Left => Point(player_pos.0 - step, player_pos.1),
            Direction::UpLeft => Point(player_pos.0 - step, player_pos.1 + step),
        };

        decrement_fuel(&e, get_move_fuel(&e) * used_fuel_mul)?;
        change_position(&e, point);

        Ok(())
    }

    /// Shoot in the current player direction.
    pub fn p_shoot(e: Env) -> Result<(), Error> {
        let range = get_laser_range(&e);
        let direction = get_direction(&e);
        let user_position = get_position(&e);

        let map = build_range_map(&e, calc_center(&e, user_position));
        let collisions = get_laser_collisions(&e, user_position, direction, range as i32);

        for wrapped_collision in collisions.iter() {
            let collision = wrapped_collision;
            if let Some(MapElement::Asteroid) = map.get(collision) {
                set_expired(&e, collision, MapElement::Asteroid);
                increment_points(&e, get_reward_amount(&e));
            }
        }

        decrement_fuel(&e, get_shoot_fuel(&e))
    }

    /// Harvest a fuel pod.
    pub fn p_harvest(e: Env) -> Result<(), Error> {
        let user_position = get_position(&e);
        let map = build_range_map(&e, calc_center(&e, user_position));

        if let Some(el) = map.get(user_position) {
            if el == MapElement::FuelPod {
                set_expired(&e, user_position, MapElement::FuelPod);
                increment_fuel(&e, 100);
            }
        }

        Ok(())
    }

    /// Upgrade the ship to get every fuel cost halfed.
    pub fn p_upgrade(e: Env) -> Result<(), Error> {
        if e.storage().instance().has(&DataKey::Upgraded) {
            return Err(Error::UnknownErr);
        }

        let curr_shoot_fuel = get_shoot_fuel(&e);
        let curr_move_fuel = get_move_fuel(&e);
        let curr_turn_fuel = get_turn_fuel(&e);

        set_shoot_fuel(&e, curr_shoot_fuel / 2);
        set_move_fuel(&e, curr_move_fuel / 2);
        set_turn_fuel(&e, curr_turn_fuel / 2);

        decrement_points(&e, 5);

        e.storage().instance().set(&DataKey::Upgraded, &true);

        Ok(())
    }

    /// Get the player's position on the grid.
    pub fn p_pos(e: Env) -> Point {
        get_position(&e)
    }

    /// Get the player's current direction.
    pub fn p_dir(e: Env) -> Direction {
        get_direction(&e)
    }

    /// Get how many points the player has currently collected.
    pub fn p_points(e: Env) -> u32 {
        get_points(&e)
    }

    /// Get how much fuel the player's ship has left.
    pub fn p_fuel(e: Env) -> u32 {
        get_fuel(&e)
    }

    /// Get the map of the current galaxy.
    pub fn get_map(e: Env) -> Map<Point, MapElement> {
        let user_position: Point = get_position(&e);
        build_range_map(&e, calc_center(&e, user_position))
    }
}
