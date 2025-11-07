use std::collections::HashMap;
use crate::errors::error::CartError;

pub struct ResourceLoader {
    resources: HashMap<String, String>,
}

impl ResourceLoader {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn add_resource(&mut self, name: &str, content: &str) -> Result<(), CartError> {
        self.resources.insert(name.to_string(), content.to_string());
        Ok(())
    }

    pub fn get_resource(&self, name: &str) -> Result<&String, CartError> {
        self.resources.get(name).ok_or(CartError::CustomError(format!("Resource '{}' not found", name)))
    }

    pub fn remove_resource(&mut self, name: &str) -> Result<(), CartError> {
        self.resources.remove(name).ok_or(CartError::CustomError(format!("Resource '{}' not found", name)))?;
        Ok(())
    }
}
