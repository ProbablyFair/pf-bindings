fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let bindings_path = std::path::Path::new(&crate_dir).join("bindings");
    std::fs::create_dir_all(&bindings_path).unwrap();
    
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        enumeration: cbindgen::EnumConfig {
            rename_variants: cbindgen::RenameRule::PascalCase,
            prefix_with_name: true,
            ..Default::default()
        },
        constant: cbindgen::ConstantConfig {
            allow_static_const: true,
            ..Default::default()
        },
        ..Default::default()
    };
    
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .unwrap()
        .write_to_file(bindings_path.join("pf_bindings.h"));
}
