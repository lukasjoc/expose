// file module
// #[path = "file.rs"]
// pub mod file;
//
// TOOD: figure out how to import this shit nicely
// mod file;
// use file::{FileNode, FileNodes};
//
//
//
//
//
//
extern crate notify;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

pub fn always(
    send: Sender<RawEvent>,
    tail: Receiver<RawEvent>,
    nodes: HashMap<String, super::file::FileNode>,
) {
    let mut watcher = raw_watcher(send).unwrap();
    let mut watchers = Vec::new();

    for path in nodes.keys() {
        watchers.push(watcher.watch(path, RecursiveMode::Recursive));
    }

    loop {
        match tail.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                // TODO: determine the operation
                //       -> CREATE
                //       -> RENAME
                //       -> CHMOD
                //       -> REMOVE
                // TODO: action on events
                //       -> unwatch paths
                //       -> watch paths
                //       -> update FileNodes
                println!("{:?} {:?}, {:?}", path, op, cookie);
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
