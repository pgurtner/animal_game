pub mod graphic_context;
pub mod keyboard_context;
pub mod timer_context;

#[cfg(test)]
mod tests {
    use super::{
        graphic_context::animal_game_engine_graphics_adapter::AnimalGameEngineGraphicContext,
        keyboard_context::animal_game_engine_keyboard_adapter::AnimalGameEngineKeyboardContext,
        timer_context::animal_game_engine_timer_adapter::AnimalGameEngineTimerContext
    };
    use animal_game::interfaces::engine::GameContext;

    #[test]
    fn creating_std_context() {
        let g_ctx = Box::new(AnimalGameEngineGraphicContext::new());
        let k_ctx = Box::new(AnimalGameEngineKeyboardContext::new());
        let t_ctx = Box::new(AnimalGameEngineTimerContext::new());
        let _ctx = GameContext::new(g_ctx, k_ctx, t_ctx);
    }
}
