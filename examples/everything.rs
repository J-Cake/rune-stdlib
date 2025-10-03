use std::io;
use std::sync::Arc;
use rune_std::Stdlib;

const EXAMPLE: &str = r#"
pub fn main() {
    println!("Hello World!");
}
"#;

type Result<Ok> = core::result::Result<Ok, Box<dyn std::error::Error>>;

pub fn main() -> Result<()> {
    let mut sources = rune::Sources::new();
    let source = rune::Source::memory(EXAMPLE)?;
    sources.insert(source)?;

    let mut diagnostics = rune::Diagnostics::new();
    let context = rune::Context::with_default_modules()?
        .stdlib()?;

    let unit = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build()?;
    let unit = Arc::new(unit);

    let rt = Arc::new(context.runtime()?);
    let mut vm = rune::Vm::new(rt, unit);

    let res = vm.call(["main"], ())?;

    Ok(())
}
