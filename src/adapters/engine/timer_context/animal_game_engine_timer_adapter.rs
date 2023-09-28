use animal_game::interfaces::engine::timer_context::TimerContext;

pub struct AnimalGameEngineTimerContext {}

impl AnimalGameEngineTimerContext {
    pub fn new() -> Self {
        return AnimalGameEngineTimerContext {};
    }
}

impl TimerContext for AnimalGameEngineTimerContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_std_timer_context() {
        let _tctx: Box<dyn TimerContext> = Box::new(AnimalGameEngineTimerContext::new());
    }
}
