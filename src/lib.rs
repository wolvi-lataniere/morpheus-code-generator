mod file_generator;
mod yaml_parser;

pub use file_generator::*;
use std::error;
use std::fs::File;
pub use yaml_parser::*;

pub struct Arguments {
    pub input: String,
    pub c_header: Option<String>,
    pub c_source: Option<String>,
    pub rust_source: Option<String>,
}

impl Arguments {
    pub fn new(argv: Vec<String>) -> Result<Arguments, &'static str> {
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

        if let Ok(matches) = options.parse(&argv[1..]) {
            Ok(Arguments {
                input: matches.opt_str("i").unwrap(),
                c_header: matches.opt_str("h"),
                c_source: matches.opt_str("c"),
                rust_source: matches.opt_str("r"),
            })
        } else {
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
            Err("No arguments found")
        }
    }
}

pub fn parse_input_file_and_generate_outputs(
    input_file: File,
    opts: Arguments,
) -> Result<(), Box<dyn error::Error>> {
    let input_file_content: yaml_parser::CodesFile = serde_yaml::from_reader(input_file)?;
    println!("Parsing OK");

    if let Some(rust) = opts.rust_source {
        println!("Generating RUST source: {}", &rust);
        let mut builder = RustFileGenerator::new(rust)?;
        builder.build_file(&input_file_content)?;
    }
    if let Some(c) = &opts.c_header {
        println!("Generating C Headers: {}", &c);
        let mut builder = CppHeaderGenerator::new(c)?;
        builder.build_file(&input_file_content)?;
    }
    if let Some(c) = &opts.c_source {
        println!("Generating C Sources: {}", &c);
        let mut builder = CppFileGenerator::new(c, &opts.c_header)?;
        builder.build_file(&input_file_content)?;
    }
    Ok(())
}
