#[macro_export]
macro_rules! impl_inherits {
    ($class:path, $inherits:path) => {
        impl<I> Deref for $class {
            type Target = $inherits;
            fn deref(&self) -> &$inherits {
                &self.superclass
            }
        }
        impl<I> DerefMut for $class {
            fn deref_mut(&mut self) -> &mut $inherits {
                &mut self.superclass
            }
        }
    };
}
