extern crate notify;

use clap::{
    crate_authors, //
    crate_description,
    crate_name,
    crate_version,
    App,
    AppSettings,
    Arg,
};
use notify::{
    raw_watcher, //
    RawEvent,
    RecursiveMode,
    Watcher,
};
use regex::Regex;
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::ffi::OsString;
use std::fs::{FileType, Metadata};
use std::path::PathBuf;
use std::sync::mpsc::channel;
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
    // root: OsString,
    nodes: HashMap<String, FileNode>,
    //node_count: usize,
    //nodes_size: u64,
}

impl FileNodes {
    pub fn new(path: &str, ignore: regex::Regex) -> Result<FileNodes, walkdir::Error> {
        let mut f: FileNodes = Default::default();
        let mut nodes = HashMap::new();
        for wd_e in WalkDir::new(path) {
            let entry = match wd_e {
                Ok(entry) => entry,
                Err(err) => return Err(err),
            };
            if !ignore.is_match(entry.file_name().to_str().unwrap()) {
                let path = entry.path().display().to_string();
                nodes.insert(path, FileNode::new(entry.to_owned()));
            }
            //else {
            //    println!(
            //        "Skipping based on ignore patttern: {:?} {:?}",
            //        ignore, entry,
            //    );
            //}
        }
        f.nodes = nodes;
        Ok(f)
    }

    // standard nodes watcher giving events
    //pub fn watcher(&mut self) -> RawEvent {
    //    let (a, b) = channel();
    //    let mut watcher = raw_watcher(a).unwrap();
    //    watcher.watch(self.root, RecursiveMode::Recursive).unwrap();
    //    b.recv().unwrap()
    //}
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
                .help("Path to dir or file to watch all desired contents recursively"),
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

    // we have nodes from path?
    // TODO: somehow give this all the paths it can search in
    // multiple wathers?
    // TODO: Ok(), Err() Handling
    let nodes = FileNodes::new(path.to_str().unwrap(), ignore)
        .unwrap()
        .nodes;

    // dec a new channel for communication
    let (head, tail) = channel();

    let mut watcher = raw_watcher(head).unwrap();
    let mut watchers = Vec::new();
    for path in nodes.keys() {
        watchers.push(
            watcher
                .watch(path, RecursiveMode::Recursive)
                .map_err(|e| format!("Err: {}", e)),
        );
    }
    loop {
        match tail.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?}", path);

                // println!("{:?} {:?} ({:?})", op, path, cookie);
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

// TODO: write tests
