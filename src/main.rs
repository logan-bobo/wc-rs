use std::process;

fn main() {
    if let Err(e) = wc_rust::run() {
        eprint!("Application Error: {e}");
        process::exit(1);
    }
}
