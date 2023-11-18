fn main() {
    let snappy = cmake::Config::new("snappy")
        .define("SNAPPY_BUILD_TESTS", "OFF")
        .define("SNAPPY_BUILD_BENCHMARKS", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", snappy.display());
    println!("cargo:rustc-link-lib=static=snappy");

    let cxx_bridge_files = vec!["src/lib.rs"];

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let include_dir = std::path::Path::new(&out_dir).join("include");
    cxx_build::bridges(cxx_bridge_files)
        .include(include_dir)
        .flag_if_supported("-fno-rtti")
        .compile("snappy-cxx-rs");
}
