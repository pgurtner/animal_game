use animal_game::interfaces::engine::graphic_context::GraphicContext;

pub struct AnimalGameEngineGraphicContext {}

impl GraphicContext for AnimalGameEngineGraphicContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graphic_context_works() {
        let _a: Box<dyn GraphicContext> = Box::new(AnimalGameEngineGraphicContext::new());
    }
}
