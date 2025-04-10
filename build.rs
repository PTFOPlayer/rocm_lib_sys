use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=rocm_smi64");

    let bindings = bindgen::Builder::default()
        .header("./rocm_smi.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap();

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
