use codes_parser::{Arguments, parse_input_file_and_generate_outputs};
use std::fs::File;

fn main() {
    let commandline_arguments: Vec<String> = std::env::args().collect();
    let opts = Arguments::new(commandline_arguments).unwrap_or_else(|_| std::process::exit(0));

    let input_file = File::open(&opts.input).expect("Error openning input file");

    parse_input_file_and_generate_outputs(input_file, opts).expect("Failed processing file");
}
