//! See [Players](Players).
use crate::ecs;
use crate::world::World;

/// Contains a list of players and manages the operations that each of them execute.
/// This is meant as an abstraction layer to keep the ECS pattern out of the program's core.
pub trait Players {}

type Player = ecs::ECSEntity;

/// todo
pub struct PlayerConfig {
    // todo
}

/// Standard implementation of [Players](Players).
pub struct StdPlayers {
    players: Vec<Player>
}

impl StdPlayers {
    /// # Arguments
    /// * world - The world in which this player list is.
    /// * players - The initial list of players.
    pub fn new (world: Box<dyn World>, players: Vec<PlayerConfig>) -> StdPlayers {
        todo!()
    }
}

impl Players for StdPlayers {}