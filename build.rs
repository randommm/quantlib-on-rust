fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/some_example.cc")
        .flag_if_supported("-std=c++14")
        .include("include/quantlib/install_dir/include")
        .compile("quantlib_on_rust");

    println!("cargo:rustc-link-search=native=include/quantlib/install_dir/lib");
    println!("cargo:rustc-link-lib=static=QuantLib");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/some_example.cc");
    println!("cargo:rerun-if-changed=include/some_example.h");
}
