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
    opts.optflag("a", "all", "print all files (including hidden ones)");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Check for options
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let filter = if matches.opt_present("a") {
        filter::all_files
    } else {
        filter::no_hidden
    };

    // Fetch given path
    let path = if matches.free.is_empty() {
        vec![String::from(".")]
    } else {
        matches.free
    };

    for p in path {
        list::print_dir(&p, filter)
    }
}
