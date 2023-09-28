mod ecs;
pub mod interfaces;
use interfaces::engine::{GameContext, GameStepResult};

pub fn draw (gctx: GameContext) -> GameStepResult {
    Ok(())
}

pub fn update (gctx: GameContext) -> GameStepResult {
    Ok(())
}