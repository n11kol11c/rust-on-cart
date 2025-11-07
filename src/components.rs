use crate::CartError;

pub trait Component: Send + Sync + 'static {
    fn setup(&mut self, _ctx: &mut crate::Context) -> Result<(), CartError> {
        Ok(())
    }
    fn start(&mut self, _ctx: &mut crate::Context) -> Result<(), CartError> {
        Ok(())
    }
    fn stop(&mut self, _ctx: &mut crate::Context) -> Result<(), CartError> {
        Ok(())
    }
}
