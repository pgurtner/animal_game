//! See [World](World).
use crate::players::Players;

/// A world is a part of a simulation. It contains multiple players.
pub trait World {
    /// Computes one single simulation step.
    fn step (&mut self);

    //todo senses
}

/// Standard implementation of [World](World).
pub struct StdWorld {
    players: Box<dyn Players>
}

impl StdWorld {
    /// # Arguments
    /// * players - The starting set of players of this world.
    pub fn new (players: Box<dyn Players>) -> StdWorld {
        StdWorld {
            players
        }
    }
}

impl World for StdWorld {
    fn step (&mut self) {
        todo!()
    }
}