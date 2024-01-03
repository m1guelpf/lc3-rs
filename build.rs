use bindgen::{CargoCallbacks, MacroTypeVariation};
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let c_files = [
        "attdet.c", "bits.c", "bwdet.c", "energy.c", "lc3.c", "ltpf.c", "mdct.c", "plc.c", "sns.c",
        "spec.c", "tables.c", "tns.c",
    ];

    let mut ccomp = cc::Build::new();

    ccomp.include("liblc3/include");
    println!("cargo:include=liblc3/include");
    println!("cargo:rustc-link-lib=static=liblc3");
    let link_dir = dst.join("lib").to_str().map(str::to_string).unwrap();
    println!("cargo:rustc-link-search={link_dir}");
    println!("cargo:static=1");

    for path in c_files {
        ccomp.file(format!("liblc3/src/{path}"));
    }

    ccomp.define("FLOATING_POINT", None).define("EXPORT", "");
    ccomp.warnings(false);
    ccomp.out_dir(dst.join("lib"));
    ccomp.compile("liblc3");

    let bindings = bindgen::Builder::default()
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .header("liblc3/include/lc3.h")
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Failed to write bindings");
}
