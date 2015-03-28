extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

static USAGE: &'static str = "
Usage: seax [-v] <bin>

Options:
    -v, --verbose   Enable verbose mode
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_bin: String,
    flag_verbose: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
