extern crate getopts;

use getopts::Options;
use std::env;

mod list;
mod filter;

/// Print usage for the current program.
///
/// - program   : the current program name
/// - opts      : array of options  to print
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // Must enforce type
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Build new Options parser
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Check for options
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.is_empty() {
        list::print_dir(&String::from("."))
    } else {
        for path in &matches.free {
            list::print_dir(path)
        }
    }
}
