pub mod animal_game_engine_timer;

pub trait TimerContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_std_timer_context() {
        let _tctx: Box<dyn TimerContext>;
    }
}
