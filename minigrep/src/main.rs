use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        process::exit(1)
    }
}
