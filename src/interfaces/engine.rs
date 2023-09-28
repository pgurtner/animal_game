pub mod graphic_context;
pub mod keyboard_context;
pub mod timer_context;

use graphic_context::GraphicContext;
use keyboard_context::KeyboardContext;
use timer_context::TimerContext;

pub type GameStepError = i32;//animal_game_engine::GameError;
pub type GameStepResult = Result<(), GameStepError>;

pub struct GameContext {
    pub graphic_context: Box<dyn GraphicContext>,
    pub keyboard_context: Box<dyn KeyboardContext>,
    pub timer_context: Box<dyn TimerContext>,
}

impl GameContext {
    pub fn new(
        gctx: Box<dyn GraphicContext>,
        kctx: Box<dyn KeyboardContext>,
        tctx: Box<dyn TimerContext>,
    ) -> Self {
        return GameContext {
            graphic_context: gctx,
            keyboard_context: kctx,
            timer_context: tctx,
        };
    }
}

