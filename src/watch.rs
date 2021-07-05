extern crate notify;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs::*;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::SystemTime;

// This is a struct with info about
// a file in the tree
// contains info about file, type, size, content ...
#[derive(Debug)]
struct FileMeta {
    modified: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
}

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    file_type: &'static str,
    file_size: u64,
    file_mod: u16,
    file_meta: FileMeta,
}

// This is the files tree
#[derive(Debug)]
pub struct Files {
    files: HashMap<&'static str, FileInfo>,
    // TODO: meta data about the hashmap
}

impl Files {
    pub fn new(path: &str) -> Self {
        let files: HashMap<&'static str, FileInfo> = HashMap::new();
        for entry in read_dir(path) {
            println!("E: {:?}", entry);
        }
        Files { files }
    }
    // return if the key is contained in the struct files
    // member
    pub fn contains_key(&self, k: &str) -> bool {
        self.files.contains_key(&k)
    }
}

// given a path watches it by an interval
pub fn changes(path: &str, interval: u64, map: Files) {
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    // keep watching the rec
    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                if map.contains_key(path.to_str().unwrap()) {
                    // update element
                    println!("Action Received: {:?} {:?} ({:?})", op, path, cookie);
                    println!("path found in map: {:?}", path);
                } else {
                    // add new element (on create actions)
                    println!("Action Received: {:?} {:?} ({:?})", op, path, cookie);
                    println!("path not in map: {:?}", path);
                }
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
