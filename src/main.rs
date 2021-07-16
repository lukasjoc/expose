mod cmd;
mod file;
mod watch;

use file::FileNodes;
use regex::Regex;
use std::env;
use std::path::PathBuf;
use std::sync::mpsc::channel;

fn main() {
    // get all matchs for args
    let opts = cmd::build_cmd().get_matches();

    // if path is matched resolve and validate it
    let path = match opts.value_of("path").unwrap() {
        "." => env::current_dir().unwrap(),
        path => PathBuf::from(path)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Cannot find the path given: {}", path)),
    };

    // if ignore is matches parse the input to a valid regex
    // pattern
    let ignore = match opts.value_of("ignore").unwrap() {
        pattern => Regex::new(pattern).unwrap(),
    };

    let nodes = FileNodes::new(path.to_str().unwrap(), ignore)
        .unwrap()
        .nodes();

    let (send, recv) = channel();

    // keep watching nodes and act on events
    watch::always(send, recv, nodes);
}
