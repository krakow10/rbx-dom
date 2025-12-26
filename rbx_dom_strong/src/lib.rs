mod class;
mod dom;
mod instance;

pub use dom::StrongDom;
pub use instance::{Instance, InstanceBuilder};

#[cfg(test)]
mod test;
