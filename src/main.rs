use std::env;
use std::path::PathBuf;
use str::parse;
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

    let interval = matches
        .value_of("interval")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let port = matches.value_of("port").unwrap().parse::<u64>().unwrap();

    let host = matches.value_of("host").unwrap();

    // TODO: Signals Handling, Watching
    watch::changes(path.display().to_string(), interval, port, host);
}
