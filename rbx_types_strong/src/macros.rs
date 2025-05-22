#[macro_export]
macro_rules! impl_inherits {
    ($class:ident,$inherits:ident) => {
        impl Deref for $class {
            type Target = $inherits;
            fn deref(&self) -> &$inherits {
                &self.superclass
            }
        }
        impl DerefMut for $class {
            fn deref_mut(&mut self) -> &mut $inherits {
                &mut self.superclass
            }
        }
    };
}
