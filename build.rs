use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/protobuf/items.proto"], &["src/protobuf/"])?;
    Ok(())
}
