pub trait GraphicContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_std_graphic_context() {
        let _gctx: Box<dyn GraphicContext>;
    }
}
