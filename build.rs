use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .warnings(true)
        .file("itersolve.c")
        .file("kin_cartesian.c")
        .file("pyhelper.c")
        .file("serialqueue.c")
        .file("stepcompress.c")
        .compile("klipper");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
