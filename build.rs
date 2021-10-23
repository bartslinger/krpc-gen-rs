use std::io::Result;
use std::path::Path;
fn main () -> Result<()> {

    let template = Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("templates").join("service.rs.hbs");
    let out_template = Path::new(&std::env::var("OUT_DIR").unwrap()).join("service.rs.hsb");
    std::fs::copy(template, out_template)?;
    Ok(())
}
