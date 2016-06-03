extern crate getopts;

use getopts::Options;
use std::env;

mod list;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}


/**
 * First parses arguments.
 * Then run the print_dir function with the given arguments.
 */
fn main() {
    // Must enforce type 
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    //list::print_dir(&args.path);
}
