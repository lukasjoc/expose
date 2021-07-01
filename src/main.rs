use std::env;
use std::path::PathBuf;

mod cmd;
mod watch;

// TODO: watch for expose.toml config file
// TODO: take default one as default config
// Eval if we need this even !
//
fn main() {
    let matches = cmd::build_cmd().get_matches();

    let path = match matches.value_of("path").unwrap() {
        "." => env::current_dir().unwrap(),
        other => PathBuf::from(other)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Cannot start watching file: {}", other)),
    };

    println!("path should be string {:?}", path);
    // TODO: Signals Handling
    //
    watch::changes(path);
}
