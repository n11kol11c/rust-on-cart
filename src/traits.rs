pub trait Executable {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait Resettable {
    fn reset(&mut self);
}
