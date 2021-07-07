use std::env;
use std::path::PathBuf;
use watch::Files;
mod cmd;
mod watch;

fn main() {
    let matches = cmd::build_cmd().get_matches();

    let path = match matches.value_of("path").unwrap() {
        "." => env::current_dir().unwrap(),
        other => PathBuf::from(other)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Cannot start watching file: {}", other)),
    };

    // TODO: build the tree from a path
    // eg. /tmp/test/test
    // /tmp/ /tmp/test /tmp/test/test -> keys in the tree
    // HahsMap<"/tmp/test/test", FileInfo{path: '/tmp/test/test', file_type: 'dir', ''}>
    //
    // to_string is not nice.. TODO: refac later
    let files_tree = watch::Files::new(path.to_str().unwrap());
    // -> files_tree
    //
    println!("{:?}", files_tree);
}
