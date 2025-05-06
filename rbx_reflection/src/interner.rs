/// A trait representing string internment that lives for at least 'host.
/// There is a blanket implementation for any function fn(&str)->&str
/// so this can be a closure operating your very own string interner.
pub trait StringIntern<'file, 'host> {
    fn intern(&mut self, str: &'file str) -> &'host str;
}

/// A type alias for a function which implements the StringIntern trait.
pub type InternFunction<'file, 'host> = fn(&'file str) -> &'host str;

impl<'file, 'host, F> StringIntern<'file, 'host> for F
where
    F: FnMut(&'file str) -> &'host str,
{
    fn intern(&mut self, str: &'file str) -> &'host str {
        self(str)
    }
}
