use crate::graphic_context::GraphicContext;
use crate::keyboard_context::KeyboardContext;
use crate::timer_context::TimerContext;

mod ecs;
mod graphic_context;
mod keyboard_context;
mod timer_context;

pub struct Context {
    pub graphic_context: Box<dyn GraphicContext>,
    pub keyboard_context: Box<dyn KeyboardContext>,
    pub timer_context: Box<dyn TimerContext>,
}

impl Context {
    pub fn new(
        gctx: Box<dyn GraphicContext>,
        kctx: Box<dyn KeyboardContext>,
        tctx: Box<dyn TimerContext>,
    ) -> Self {
        return Context {
            graphic_context: gctx,
            keyboard_context: kctx,
            timer_context: tctx,
        };
    }
}

#[ecs::component]
struct MyTestStruct {}

#[ecs::system]
fn my_test_function (a: &mut MyTestStruct) {

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic_context::animal_game_engine_graphics::AnimalGameEngineGraphicContext;
    use crate::keyboard_context::animal_game_engine_keyboard::AnimalGameEngineKeyboardContext;
    use crate::timer_context::animal_game_engine_timer::AnimalGameEngineTimerContext;

    #[test]
    fn creating_std_context() {
        let g_ctx = Box::new(AnimalGameEngineGraphicContext::new());
        let k_ctx = Box::new(AnimalGameEngineKeyboardContext::new());
        let t_ctx = Box::new(AnimalGameEngineTimerContext::new());
        let _ctx = Context::new(g_ctx, k_ctx, t_ctx);
    }
}
