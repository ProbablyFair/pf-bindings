fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        ..Default::default()
    };
    
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .unwrap()
        .write_to_file("packages/dart/include/pf_bindings.h");
}
