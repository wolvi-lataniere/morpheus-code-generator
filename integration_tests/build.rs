use std::env;
use std::fs::File;
use std::path::PathBuf;

use codes_parser::Arguments;

fn build(source_file: &String) {
    cc::Build::new()
        .cpp(true)
        .include(env::var("OUT_DIR").unwrap())
        .include("./c")
        .file(source_file)
        .compile("generated");
}

fn generate_bindings(header_file: &String) {
    let bindings = bindgen::Builder::default()
        .header(header_file)
        .clang_args(["-x", "c++"])
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path_string = out_path.to_str().unwrap();
    let input_file = File::open("test.yml").unwrap();
    let output_c_source: String = out_path.join("test_output.cpp").to_str().unwrap().into();
    let output_c_header: String = out_path.join("test_output.h").to_str().unwrap().into();
    let output_rs: String = out_path.join("test_output.rs").to_str().unwrap().into();
    let opts = Arguments {
        c_header: Some(output_c_header.clone()),
        c_source: Some(output_c_source.clone()),
        rust_source: Some(output_rs),
        input: "test.yml".into(),
    };
    codes_parser::parse_input_file_and_generate_outputs(input_file, opts).unwrap();
    build(&output_c_source);
    generate_bindings(&output_c_header);

    println!("cargo::rustc-link-search=native={out_path_string}");
    println!("cargo::rustc-link-lib=generated");
    println!("cargo::rerun-if-changed=test.yml");
}
