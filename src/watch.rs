extern crate notify;
// use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs::Metadata;
// use std::io;
use std::ffi::OsStr;
use std::path::Path;
// use std::path::PathBuf;
// use std::sync::mpsc::channel;
use std::time::SystemTime;
use walkdir::WalkDir;

// This is a struct with info about
// a file in the tree
// contains info about file, type, size, content ...
#[derive(Debug)]
struct FileInfo<'a> {
    path: &'a OsStr,
    is_file: bool,
    is_dir: bool,
    //     is_simlink: bool,
    file_size: u64,
    file_mode: std::fs::Permissions,
    modified: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
}

impl<'a> FileInfo<'a> {
    pub fn new(path: &'a OsStr, entry: Metadata) -> Self {
        FileInfo {
            path: path,
            is_file: entry.is_file(),
            is_dir: entry.is_dir(),
            //             is_simlink: entry.is_link(),
            file_size: entry.len() / 1024 / 1024,
            file_mode: entry.permissions(),
            modified: entry.modified().unwrap(),
            accessed: entry.accessed().unwrap(),
            created: entry.created().unwrap(),
        }
    }
}

// This is the files tree
#[derive(Debug)]
pub struct Files<'a> {
    files: HashMap<&'a OsStr, FileInfo<'a>>,
    // TODO: meta data about the hashmap
}

impl<'a> Files<'a> {
    // generate new files struct recursively based on the path given to it
    // this generates a static implementation of the
    // contents in the given path and subpaths
    pub fn new(path: &str) -> Self {
        let mut path_tree: HashMap<&OsStr, FileInfo> = HashMap::new();
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            for meta in std::fs::metadata(entry.path()) {
                path_tree.insert(OsStr::new("/"), FileInfo::new(OsStr::new("/"), meta));

                // path_tree.insert(entry.unwrap(), FileInfo::new(entry_file_name, meta));
            }
        }
        Files { files: path_tree }
    }
    // TODO:
    // update a key
    // delete a key
    // create a key
    // query a key
}

// given a path watches it by an interval
//pub fn changes(path: &str, map: Files) {
//    let (tx, rx) = channel();
//    let mut watcher = raw_watcher(tx).unwrap();
//    watcher.watch(path, RecursiveMode::Recursive).unwrap();
//
//    // keep watching the rec
//    loop {
//        match rx.recv() {
//            Ok(RawEvent {
//                path: Some(path),
//                op: Ok(op),
//                cookie,
//            }) => {
//                if map.contains_key(path.to_str().unwrap()) {
//                    // update element
//                    println!("Action Received: {:?} {:?} ({:?})", op, path, cookie);
//                    println!("path found in map: {:?}", path);
//                } else {
//                    // add new element (on create actions)
//                    println!("Action Received: {:?} {:?} ({:?})", op, path, cookie);
//                    println!("path not in map: {:?}", path);
//                }
//            }
//            Ok(event) => println!("broken event: {:?}", event),
//            Err(e) => println!("watch error: {:?}", e),
//        }
//    }
//}
