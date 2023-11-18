fn main() {
    let snappy = cmake::Config::new("snappy")
        .define("SNAPPY_BUILD_TESTS", "OFF")
        .define("SNAPPY_BUILD_BENCHMARKS", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", snappy.display());
    println!("cargo:rustc-link-lib=static=snappy");
    println!("cargo:rustc-link-lib=stdc++");

    let snappy_h = snappy.join("include/snappy-c.h");
    let snappy_h = snappy_h.to_str().unwrap();
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("bindings.rs");

    bindgen::Builder::default()
        .header(snappy_h)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
