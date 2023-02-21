use soroban_sdk::Env;

use crate::types::{DataKey, Direction, Error, MapElement, Point};

pub fn get_seed(e: &Env) -> u64 {
    e.storage().get(&DataKey::Seed).unwrap().unwrap()
}

pub fn change_position(e: &Env, position: Point) {
    e.storage().set(&DataKey::PlayerPos, &position);
}

pub fn get_position(e: &Env) -> Point {
    e.storage().get(&DataKey::PlayerPos).unwrap().unwrap()
}
pub fn change_direction(e: &Env, direction: Direction) {
    e.storage().set(&DataKey::PlayerDir, &direction);
}

pub fn get_direction(e: &Env) -> Direction {
    e.storage().get(&DataKey::PlayerDir).unwrap().unwrap()
}

pub fn get_step(e: &Env) -> u32 {
    e.storage().get(&DataKey::MoveStep).unwrap().unwrap()
}

pub fn get_laser_range(e: &Env) -> u32 {
    e.storage().get(&DataKey::LaserRange).unwrap().unwrap()
}

pub fn get_range(e: &Env) -> u32 {
    e.storage().get(&DataKey::Range).unwrap().unwrap()
}

pub fn get_reward_amount(e: &Env) -> u32 {
    e.storage().get(&DataKey::Reward).unwrap().unwrap()
}

pub fn get_move_fuel(e: &Env) -> u32 {
    e.storage().get(&DataKey::MoveFuel).unwrap().unwrap()
}

pub fn get_turn_fuel(e: &Env) -> u32 {
    e.storage().get(&DataKey::TurnFuel).unwrap().unwrap()
}

pub fn get_shoot_fuel(e: &Env) -> u32 {
    e.storage().get(&DataKey::ShootFuel).unwrap().unwrap()
}

pub fn get_points(e: &Env) -> u32 {
    e.storage().get(&DataKey::Points).unwrap().unwrap()
}

pub fn increment_points(e: &Env, points: u32) {
    e.storage().set(&DataKey::Points, &(get_points(e) + points)); // so that we can also subtract points
}

pub fn decrement_points(e: &Env, points: u32) {
    e.storage().set(&DataKey::Points, &(get_points(e) - points)); // so that we can also subtract points
}

pub fn get_fuel(e: &Env) -> u32 {
    e.storage().get(&DataKey::PlayerFuel).unwrap().unwrap()
}

pub fn decrement_fuel(e: &Env, amount: u32) -> Result<(), Error> {
    let current_fuel = get_fuel(e);

    if current_fuel as i32 - (amount as i32) < 0i32 {
        return Err(Error::NotEnoughFuel);
    }

    e.storage()
        .set(&DataKey::PlayerFuel, &(current_fuel - amount));

    Ok(())
}

pub fn increment_fuel(e: &Env, amount: u32) {
    let current_fuel = get_fuel(e);
    e.storage()
        .set(&DataKey::PlayerFuel, &(current_fuel + amount));
}

pub fn set_shoot_fuel(e: &Env, fuel: u32) {
    e.storage().set(&DataKey::ShootFuel, &fuel);
}

pub fn set_move_fuel(e: &Env, fuel: u32) {
    e.storage().set(&DataKey::MoveFuel, &fuel);
}

pub fn set_turn_fuel(e: &Env, fuel: u32) {
    e.storage().set(&DataKey::TurnFuel, &fuel);
}

pub fn set_expired(e: &Env, point: Point, element: MapElement) {
    e.storage().set(&DataKey::Expired(point), &element);
}
