use animal_game::interfaces::engine::keyboard_context::KeyboardContext;

pub struct AnimalGameEngineKeyboardContext {}

impl AnimalGameEngineKeyboardContext {
    pub fn new() -> Self {
        return AnimalGameEngineKeyboardContext {};
    }
}

impl KeyboardContext for AnimalGameEngineKeyboardContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graphic_context_works() {
        let _a: Box<dyn KeyboardContext> = Box::new(AnimalGameEngineKeyboardContext::new());
    }
}
