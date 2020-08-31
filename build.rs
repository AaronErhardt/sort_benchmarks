extern crate bindgen;

use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("c_files/sort_c.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("c_bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("c_files/sort_c.c")
        .opt_level(3)
        .compile("sort_c");
}
