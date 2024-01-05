// Libs
use std::error::Error;

// Main
fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("./proto/mailer.proto")?;
    Ok(())
}
