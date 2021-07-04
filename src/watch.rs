extern crate notify;
use notify::{watcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::SystemTime;

// This is a struct with info about
// a file in the tree
// contains info about file, type, size, content ...
#[derive(Debug)]
pub struct FileInfo {
    path: PathBuf,
    ftype: String,
    fsize: u64,
    fmod: u16,
    modified: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
}

// This is the files tree
#[derive(Debug)]
pub struct Files {
    files: Vec<FileInfo>,
    // meta
    // TODO: accumulated size
    // number of files
    // number of dirs
    // ...
}

pub fn changes(path: PathBuf, interval: u64, port: u64, hostname: &str) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(interval)).unwrap();

    let watch_path: String = path.display().to_string();

    watcher.watch(watch_path, RecursiveMode::Recursive).unwrap();

    // keep watching the rec
    loop {
        match rx.recv() {
            //Ok(RawEvent {
            //    path: Some(path),
            //    op: Ok(op),
            //    cookie,
            //}) => {
            //    println!("=> RAW: {:?} {:?} ({:?})", op, path, cookie);
            //}
            // Ok(event) => println!("=> BROKEN: broken event: {:?}", event),
            //Err(e) => println!("=> ERR: watch error: {:?}", e),
        }
    }
}
