use std::{thread::available_parallelism, env::current_dir};

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/some_example.cc");
    println!("cargo:rerun-if-changed=include/some_example.h");

    let n_threads = available_parallelism()
        .map(|x| x.into())
        .unwrap_or(0_usize)
        .max(2);

    // build Quantlib if it doesn't exist
    // check if install_dir exists
    let install_dir_path = "include/quantlib/install_dir";
    if !std::path::Path::new(install_dir_path).exists() {
        // check if makefile exists
        let makefile_path = "include/quantlib/Makefile";
        if !std::path::Path::new(makefile_path).exists() {
            // check if configure exists
            let configure_path = "include/quantlib/configure";
            if !std::path::Path::new(configure_path).exists() {
                // run autogen.sh
                println!(">> Running autogen.sh");
                let status = std::process::Command::new("./autogen.sh")
                    .current_dir("include/quantlib")
                    .status()
                    .expect("failed to run autogen.sh");
                assert!(status.success());
                println!(">> Finished running autogen.sh");
            }

            // run configure
            let prefix = format!("--prefix={}", current_dir().unwrap().join("include/quantlib/install_dir").to_str().unwrap());
            println!(">> Running configure {prefix}");
            let status = std::process::Command::new("./configure")
                .arg(prefix)
                .current_dir("include/quantlib")
                .status()
                .expect("failed to run configure");
            assert!(status.success());
            println!(">> Finished running configure");
        }

        // run make
        println!(">> Running make");
        let status = std::process::Command::new("make")
            .args(["-j", n_threads.to_string().as_str()])
            .current_dir("include/quantlib")
            .status()
            .expect("failed to run make");
        assert!(status.success());
        println!(">> Finished running make");

        // run make install
        println!(">> Running make install");
        let status = std::process::Command::new("make")
            .arg("install")
            .current_dir("include/quantlib")
            .status()
            .expect("failed to run make");
        assert!(status.success());
        println!(">> Finished running make install");
    }

    cxx_build::bridge("src/main.rs")
        .file("src/some_example.cc")
        .flag_if_supported("-std=c++14")
        .include("include/quantlib/install_dir/include")
        .compile("quantlib_on_rust");

    println!("cargo:rustc-link-search=native=include/quantlib/install_dir/lib");
    println!("cargo:rustc-link-lib=static=QuantLib");
}
