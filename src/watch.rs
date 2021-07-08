use std::collections::HashMap;
use std::fs::Metadata;
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;

// This is a struct with info about
// a file in the tree
// contains info about file, type, size, content ...
#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    is_file: bool,
    is_dir: bool,
    file_size: u64,
    file_mode: std::fs::Permissions,
    modified: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
}

impl FileInfo {
    pub fn new(path: PathBuf, entry: Metadata) -> Self {
        FileInfo {
            path,
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
pub struct Files {
    files: HashMap<&'static str, FileInfo>,
    // TODO: meta data about the hashmap
}

impl Files {
    // generate new files struct recursively based on the path given to it
    // this generates a static implementation of the
    // contents in the given path and subpaths
    // AS ALWAYS: Fix the syntax later with rust
    pub fn collect(&mut self) -> HashMap<&'static str, FileInfo> {
        return self.files;
    }

    pub fn new(path: &str) -> Self {
        let mut path_tree: HashMap<&'static str, FileInfo> = HashMap::new();
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            for meta in std::fs::metadata(entry.path().to_owned()) {
                let path_key = entry.path().display().to_string().as_ref();
                path_tree.insert(path_key, FileInfo::new(entry.path().to_owned(), meta));
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
