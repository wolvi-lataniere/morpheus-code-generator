use getopts;
use serde_yaml;
use std::{fs::File, option, process::exit};

mod file_generator;
mod rust_template;
mod yaml_parser;

use file_generator::*;

struct Arguments {
    pub input: String,
    pub c_header: Option<String>,
    pub c_source: Option<String>,
    pub rust_source: Option<String>,
}

fn parse_arguments(argv: Vec<String>) -> Result<Arguments, ()> {
    let mut options = getopts::Options::new();
    options.optopt("r", "rust", "Rust target file", "RUST_TARGET");
    options.optopt("c", "csource", "C source target file", "C_SOURCE");
    options.optopt("h", "cheader", "C Header target file", "C_HEADER");
    options.opt(
        "i",
        "input",
        "Protocol description input file",
        "INPUT",
        getopts::HasArg::Yes,
        getopts::Occur::Req,
    );

    let matches = match options.parse(&argv[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!(
                "{}",
                options.usage(
                    format!(
                        "Usage: {} -i INPUT [-r RUST_TARGET] [-c C_SOURCE] [-c C_HEADER]",
                        argv[0]
                    )
                    .as_str()
                )
            );
            return Err(());
        }
    };

    Ok(Arguments {
        input: matches.opt_str("i").unwrap(),
        c_header: matches.opt_str("h"),
        c_source: matches.opt_str("c"),
        rust_source: matches.opt_str("r"),
    })
}

fn parse_input_file_and_generate_outputs(
    input_file: File,
    opts: Arguments,
) -> Result<(), serde_yaml::Error> {
    let input_file_content: yaml_parser::CodesFile = serde_yaml::from_reader(input_file)?;
    println!("Parsing OK");

    if let Some(rust) = opts.rust_source {
        println!("Generating RUST source: {}", &rust);
        rust_template::build_rust_source(rust, &input_file_content).unwrap();
    }
    if let Some(c) = &opts.c_header {
        println!("Generating C Headers: {}", &c);
        let mut builder =
            CppHeaderGenerator::new(&c).expect("Failed to generate Cpp Header");
        builder.build_file(&input_file_content);
    }
    if let Some(c) = &opts.c_source {
        println!("Generating C Sources: {}", &c);
        build_cpp_source(&c, &opts.c_header, &input_file_content).unwrap();
    }
    Ok(())
}

fn main() -> Result<(), serde_yaml::Error> {
    let commandline_arguments: Vec<String> = std::env::args().collect();
    let opts = parse_arguments(commandline_arguments).unwrap_or_else(|e| std::process::exit(0));

    let input_file = File::open(&opts.input);

    if let Ok(input_file) = input_file {
        parse_input_file_and_generate_outputs(input_file, opts);
    } else {
        eprintln!("Error reading file.");
    }

    Ok(())
}
