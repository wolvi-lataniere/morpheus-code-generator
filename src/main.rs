use std::{fs::File, option, process::exit};
use serde_yaml;
use getopts;


mod yaml_parser;
mod rust_template;
mod cpp_template;

fn main() -> Result<(), serde_yaml::Error> {
    let mut options = getopts::Options::new();
    options.optopt("r", "rust", "Rust target file", "RUST_TARGET");
    options.optopt("c", "csource", "C source target file", "C_SOURCE");
    options.optopt("h", "cheader", "C Header target file", "C_HEADER");
    options.opt("i", "input", "Protocol description input file", "INPUT", getopts::HasArg::Yes, getopts::Occur::Req);
 
    let args: Vec<String> = std::env::args().collect();

    let matches = match options.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { 
            println!("{}", options.usage(format!("Usage: {} -i INPUT [-r RUST_TARGET] [-c C_SOURCE] [-c C_HEADER]", args[0]).as_str()));
            return Ok(());
        }
    };

    let input: Option<String> = matches.opt_str("i");
    let c_header: Option<String> = matches.opt_str("h");
    let c_source: Option<String> = matches.opt_str("c");
    let rust_source: Option<String> = matches.opt_str("r");

    if let Ok(file) = File::open(input.unwrap()) {
        let content : yaml_parser::CodesFile = serde_yaml::from_reader(file)?;
        println!("Parsing OK");

        if let Some(rust) = rust_source{
            println!("Generating RUST source: {}", &rust);
            rust_template::build_rust_source(rust, &content).unwrap();
        }
        if let Some(c) = &c_header {
            println!("Generating C Headers: {}", &c);
            cpp_template::build_cpp_header(&c, &content);
        }
        if let Some(c) = &c_source {
            println!("Generating C Sources: {}", &c);
            cpp_template::build_cpp_source(&c, &c_header, &content).unwrap();
        } 
    } else 
    {
        eprintln!("Error reading file.");
    }

    Ok(())
}
