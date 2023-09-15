use crate::graphic_context::GraphicContext;

pub struct AnimalGameEngineGraphicContext {}

impl AnimalGameEngineGraphicContext {
    pub fn new() -> Self {
        return AnimalGameEngineGraphicContext {};
    }
}

impl GraphicContext for AnimalGameEngineGraphicContext {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic_context::GraphicContext;

    #[test]
    fn graphic_context_works() {
        let _a: Box<dyn GraphicContext> = Box::new(AnimalGameEngineGraphicContext::new());
    }
}
