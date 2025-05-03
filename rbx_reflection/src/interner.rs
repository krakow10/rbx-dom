/// A trait representing string internment that lives for at least 'host.
/// There is a blanket implementation for any function fn(&str)->&str
/// so this can be a closure operating your very own string interner.
pub trait StringIntern<'host> {
    fn intern(&mut self, str: &str) -> &'host str;
}

/// A type alias for a function which implements the StringIntern trait.
pub type InternFunction<'host> = fn(&str) -> &'host str;

impl<'host, F> StringIntern<'host> for F
where
    F: FnMut(&str) -> &'host str,
{
    fn intern(&mut self, str: &str) -> &'host str {
        self(str)
    }
}
