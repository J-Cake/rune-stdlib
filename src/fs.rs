use std::io::Read;
use rune::{Any, ContextError, Module};

#[derive(Any, Debug)]
#[rune(item = ::std::fs)]
struct File(std::fs::File);

impl File {

}

#[rune::module(::std::fs)]
pub fn module() -> Result<Module, ContextError> {
    let mut r#mod = Module::from_meta(self::module_meta)?;

    r#mod.ty::<File>()?;

    Ok(r#mod)
}