use extism::{Manifest, Plugin};

fn main() {
    const data: &[u8] = include_bytes!("plugin.wasm");
    let manifest = Manifest::new([extism::manifest::Wasm::Data {
        data: data.to_vec(),
        meta: Default::default(),
    }])
    .with_allowed_host("*")
    .with_allowed_path(".", "/app");
    let mut plugin = Plugin::create_with_manifest(&manifest, [], true).unwrap();
    let _ = plugin.call("write_file", b"this is a test").unwrap();
}
