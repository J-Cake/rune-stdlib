mod fs;

pub trait Stdlib {
    fn stdlib(self) -> Result<Self, rune::ContextError> where Self: Sized;
}

impl Stdlib for rune::Context {
    fn stdlib(mut self) -> Result<Self, rune::ContextError> {
        self.install(fs::module()?)?;
        
        Ok(self)
    }
}
