extern crate notify;

use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;

// !TODO: implement
//pub struct Watcher {
//
//}

//impl Watcher {
//}

// !TODO: implement
pub fn changes(path: PathBuf) {
    //-> Result<()> {
    // rec, trans
    let (tx, rx) = channel();

    let mut watcher = raw_watcher(tx).unwrap();

    watcher
        .watch(path.display().to_string(), RecursiveMode::Recursive)
        .unwrap();

    // keep watching the rec
    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("=> RAW: {:?} {:?} ({:?})", op, path, cookie)
            }
            Ok(event) => println!("=> BROKEN: broken event: {:?}", event),
            Err(e) => println!("=> ERR: watch error: {:?}", e),
        }
    }
}
