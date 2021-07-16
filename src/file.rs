use std::collections::HashMap;
use std::default::Default;
use std::ffi::OsString;
use std::fs::{FileType, Metadata};
use std::path::{Path, PathBuf};
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
    // root: OsString,
    // TODO: why the fuck is this key a String
    // and not a OsString
    nodes: HashMap<String, FileNode>,
    //node_count: usize,
    //nodes_size: u64,
}

impl FileNodes {
    pub fn nodes(self) -> HashMap<String, FileNode> {
        self.nodes
    }

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
            } else {
                println!(
                    "Skipping based on ignore patttern: {:?} {:?}",
                    ignore, entry,
                );
            }
        }
        f.nodes = nodes;
        Ok(f)
    }
}

/// taken  from zola yooo thanks
/// Returns whether the path we received corresponds to a temp file created
/// by an editor or the OS
fn is_temp_file(path: &Path) -> bool {
    match path.extension() {
        Some(ex) => match ex.to_str().unwrap() {
            "swp" | "swx" | "tmp" | ".DS_STORE" => true,
            x if x.ends_with('~') => true,
            _ => {
                if let Some(filename) = path.file_stem() {
                    // emacs
                    let name = filename.to_str().unwrap();
                    name.starts_with('#') || name.starts_with(".#")
                } else {
                    false
                }
            }
        },
        None => true,
    }
}
