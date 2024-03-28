use crate::{
    storage::{get_range, get_seed},
    types::{DataKey, Direction, MapElement, Point},
};
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use soroban_sdk::{Env, Map, Vec};

fn calc(e: &Env, x: i32) -> i32 {
    let range = get_range(e) as i32;
    let div = x / range;
    let rem = x % range;

    let mut center = if div > rem {
        (range + 1) * (div - 1) + range / 2
    } else {
        (range + 1) * (div) + range / 2
    };

    loop {
        if center.abs() > x.abs() + (range / 2) {
            if x > 0 {
                center -= range + 1;
            } else {
                center += range + 1;
            }
        } else {
            break;
        }
    }

    center
}

pub fn calc_center(e: &Env, point: Point) -> Point {
    Point(calc(e, point.0), calc(e, point.1))
}

fn access_rng(rng: &mut SmallRng, range_center: Point, range: i32) -> (i32, i32) {
    let x = rng.gen_range((range_center.0 - range / 2)..=(range_center.0 + range / 2));
    let y = rng.gen_range((range_center.1 - range / 2)..=(range_center.1 + range / 2));

    (x, y)
}

pub fn build_range_map(e: &Env, range_center: Point) -> Map<Point, MapElement> {
    let mut map = Map::new(e);
    let fixed_seed = get_seed(e);
    let mut rng =
        SmallRng::seed_from_u64(((range_center.0 * fixed_seed as i32) + range_center.1) as u64);

    let asteroid_range_density: u32 = e.storage().instance().get(&DataKey::AstDensity).unwrap(); // 6 asteroids in the 16x16 galaxy grid
    let fuel_pod_range_density: u32 = e.storage().instance().get(&DataKey::PodDensity).unwrap(); // 2 fuel pods in the 16x16 galaxy grid

    let range = get_range(e) as i32;

    for _ in 0..asteroid_range_density {
        let g = access_rng(&mut rng, range_center, range);
        let point = Point(g.0, g.1);

        if !e.storage().instance().has(&DataKey::Expired(point)) {
            map.set(point, MapElement::Asteroid)
        }
    }

    for _ in 0..fuel_pod_range_density {
        let g = access_rng(&mut rng, range_center, range);
        let point = Point(g.0, g.1);

        if !e.storage().instance().has(&DataKey::Expired(point)) {
            map.set(point, MapElement::FuelPod)
        }
    }

    map
}

pub fn get_laser_collisions(
    e: &Env,
    user_position: Point,
    direction: Direction,
    range: i32,
) -> Vec<Point> {
    let mut collisions: Vec<Point> = Vec::new(e);
    match direction {
        Direction::Up => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0, user_position.1 + n))
            }
        }
        Direction::UpRight => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 + n, user_position.1 + n))
            }
        }
        Direction::Right => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 + n, user_position.1))
            }
        }
        Direction::DownRight => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 + n, user_position.1 - n))
            }
        }
        Direction::Down => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0, user_position.1 - n))
            }
        }
        Direction::DownLeft => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 - n, user_position.1 - n))
            }
        }
        Direction::Left => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 - n, user_position.1))
            }
        }
        Direction::UpLeft => {
            for n in 0..=range {
                collisions.push_back(Point(user_position.0 - n, user_position.1 + n))
            }
        }
    };

    collisions
}
