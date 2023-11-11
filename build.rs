fn main() {
    let snappy = cmake::Config::new("snappy")
        .define("SNAPPY_BUILD_TESTS", "OFF")
        .define("SNAPPY_BUILD_BENCHMARKS", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", snappy.display());
    println!("cargo:rustc-link-lib=static=snappy");
    println!("cargo:rustc-link-lib=stdc++");
}
