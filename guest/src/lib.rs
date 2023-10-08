use extism_pdk::*;

#[plugin_fn]
pub fn write_file(input: String) -> FnResult<()> {
    std::fs::write("/app/text.txt", input).unwrap();
    Ok(())
}
