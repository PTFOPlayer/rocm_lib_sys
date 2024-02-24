extern crate cmake;

use cmake::Config;


fn main() {

    let dst = Config::new("rsmi")
        .define("ROCM_DIR", "/opt/rocm".to_owned())
        .very_verbose(true)
        .build();

    println!("cargo:warning={}", dst.display());
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=rocm_smi64");
    println!("cargo:rustc-link-lib=rsmi64");
}
