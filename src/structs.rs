use crate::traits::{Executable, Resettable};
use crate::result::CartResult;

pub struct Cart {
    pub name: String,
    pub items: Vec<String>,
}
