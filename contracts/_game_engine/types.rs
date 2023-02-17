use soroban_sdk::{contracterror, contracttype};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    UnknownErr = 0,
    NotEnoughFuel = 1,
}

#[contracttype]
#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

#[contracttype]
pub enum DataKey {
    MoveStep,
    LaserRange,
    Seed,
    Range,
    ShootFuel,
    Reward,
    MoveFuel,
    TurnFuel,
    PlayerFuel,
    PlayerDir,
    PlayerPos,
    Points,
    Expired(Point),
    AstDensity,
    PodDensity,
    Upgraded,
}

#[contracttype]
#[derive(Clone, Eq, PartialEq)]
pub enum MapElement {
    Asteroid,
    FuelPod,
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl core::fmt::Debug for Point {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.0)
            .field("y", &self.1)
            .finish()
    }
}

impl core::fmt::Debug for MapElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let out = match &self {
            MapElement::Asteroid => "Asteroid",
            MapElement::FuelPod => "Fuel Pod",
        };
        f.debug_struct("Element").field("type", &out).finish()
    }
}
