use std::sync::{Arc, Mutex};

use crate::{Component, Context, CartError};

pub struct Cart {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl Cart {
    pub fn builder() -> CartBuilder {
        CartBuilder {
            name: "cart-app".to_string(),
            components: vec![],
        }
    }

    pub fn run(mut self) -> Result<(), CartError> {
        let mut ctx = Context::new();

        for component in self.components.iter_mut() {
            component.setup(&mut ctx)?;
        }
        for component in self.components.iter_mut() {
            component.start(&mut ctx)?;
        }
        for component in self.components.iter_mut() {
            component.stop(&mut ctx)?;
        }

        Ok(())
    }
}

pub struct CartBuilder {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl CartBuilder {
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.name = n.into();
        self
    }

    pub fn with_component<C: Component>(mut self, c: C) -> Self {
        self.components.push(Box::new(c));
        self
    }

    pub fn build(self) -> Cart {
        Cart {
            name: self.name,
            components: self.components,
        }
    }

    pub fn run(self) -> Result<(), CartError> {
        self.build().run()
    }
}
