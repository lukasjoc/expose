use clap::{
    crate_authors, //
    crate_description,
    crate_name,
    crate_version,
    App,
    AppSettings,
    Arg,
};

use regex::Regex;
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::ffi::OsString;
use std::fs::{FileType, Metadata};
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileNode {
    path: PathBuf,
    name: OsString,
    is_dir: bool,
    is_file: bool,
    is_link: bool,
    depth: usize,
    file_size: u64,
    modified: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
}

// a standard M length
// const STD_BYTE: u64 = 1024;

impl FileNode {
    pub fn new(d: walkdir::DirEntry) -> Self {
        let meta: Metadata = d.metadata().unwrap();
        let ft: FileType = meta.file_type();
        FileNode {
            path: d.path().to_owned(),
            name: d.file_name().to_owned(),
            is_dir: ft.is_dir(),
            is_file: ft.is_file(),
            is_link: ft.is_symlink(),
            depth: d.depth(),
            modified: meta.modified().unwrap(),
            accessed: meta.accessed().unwrap(),
            created: meta.created().unwrap(),
            file_size: meta.len(),
        }
    }
}

#[derive(Debug, Default)]
pub struct FileNodes {
    // the file tree with key, value (path, entryData)
    // eg /tmp/test : {name: "/tmp/test", isDir: true}
    nodes: HashMap<String, FileNode>,
    //node_count: usize,
    //nodes_size: u64,
}

impl FileNodes {
    pub fn new(path: &str, ignore: regex::Regex) -> Self {
        let mut f: FileNodes = Default::default();
        let mut nodes = HashMap::new();
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();

            if !ignore.is_match(entry.file_name().to_str().unwrap()) {
                let path = entry.path().display().to_string();
                nodes.insert(path, FileNode::new(entry.to_owned()));
            } else {
                println!(
                    "Skipping based on ignore patttern: {:?} {:?}",
                    ignore, entry,
                );
            }
        }
        f.nodes = nodes;
        f
    }
}

fn main() {
    let opts = App::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .required(true)
                .takes_value(true)
                .help("Path to file or dir to keep watch in"),
        )
        .arg(
            Arg::with_name("ignore")
                .short("i")
                .long("ignore")
                .takes_value(true)
                .default_value(r"^\..*")
                .help(r"Ignore certain files using a defined regex pattern"),
        )
        .get_matches();

    let path = match opts.value_of("path").unwrap() {
        "." => env::current_dir().unwrap(),
        path => PathBuf::from(path)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Cannot find the path given: {}", path)),
    };

    let ignore = match opts.value_of("ignore").unwrap() {
        pattern => Regex::new(pattern).unwrap(),
    };

    println!("{:?}", ignore);
    let file_nodes = FileNodes::new(path.to_str().unwrap(), ignore);
    println!("given : {:?} {:?}", path, file_nodes);
}

// TODO: write tests
