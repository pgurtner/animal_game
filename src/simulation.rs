//! See [Simulation](Simulation).

use crate::world::World;

/// Simulation is the outermost entity that contains worlds which have players.
pub trait Simulation {
    /// Starts the simulation and lets it run forever.
    fn start (&mut self);

    /// Executes the simulation steps times.
    ///
    /// # Arguments
    /// * `steps` - Amount of simulation steps to be calculated.
    fn run_steps (&mut self, steps: u32);

    /// Computes one single simulation step.
    fn step (&mut self);

    /// Creates a dump of the simulation.
    ///
    /// # notes
    /// Currently this is *not* thread-safe.
    fn dump (&self) -> SimulationState;
}

/// Opaque struct for simulation state snapshots.
pub struct SimulationState {}

/// Standard implementation of [Simulation](Simulation).
pub struct StdSimulation {
    worlds: Vec<Box<dyn World>>,
}

impl StdSimulation {
    /// # Arguments
    /// * worlds - The worlds this simulation consists of.
    pub fn new (worlds: Vec<Box<dyn World>>) -> StdSimulation {
        StdSimulation {
            worlds
        }
    }
}

impl Simulation for StdSimulation {
    fn start(&mut self) {
        todo!()
    }

    fn run_steps(&mut self, steps: u32) {
        todo!()
    }

    fn step(&mut self) {
        todo!()
    }

    fn dump(&self) -> SimulationState {
        todo!()
    }
}