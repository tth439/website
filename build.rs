use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env().unwrap();
    let mut statics = ructe.statics().unwrap();
    statics.add_files("ui/dist").unwrap();
    ructe.compile_templates("ui/templates").unwrap();
    Ok(())
}
