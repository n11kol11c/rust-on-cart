use cart::{Cart, CartBuilder, Component, Context};

#[derive(Component)]
struct Logger;

impl Component for Logger {
    fn start(&mut self, _ctx: &mut Context) -> Result<(), cart::CartError> {
        println!("Logger started!");
        Ok(())
    }
}

fn main() -> Result<(), cart::CartError> {
    CartBuilder::builder()
        .name("single-crate-cart")
        .with_component(Logger)
        .run()
}
