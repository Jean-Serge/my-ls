mod list;

/**
 * First parses arguments.
 * Then run the print_dir function with the given arguments.
 */
fn main() {
  let args = list::parse_args();
  list::print_dir(&args.path);
}
