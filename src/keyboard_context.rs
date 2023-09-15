pub mod animal_game_engine_keyboard;

pub trait KeyboardContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_std_keyboard_context() {
        let _kctx: Box<dyn KeyboardContext>;
    }
}
