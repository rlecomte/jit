use crate::store::*;
use std::fs::{self, DirBuilder, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

pub(crate) struct Workspace {
    pub path: PathBuf,
    pub store: Store,
}

impl Workspace {
    pub fn initialize(&self) -> io::Result<i32> {
        let p = &self.path.canonicalize()?;
        println!("Initialize empty jit repo in {}", p.display());

        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder.create(p.join(".jit/objects"))?;
        builder.create(p.join(".jit/refs"))?;
        Ok(0)
    }

    pub fn commit(&self) -> io::Result<i32> {
        //TODO don't unwrap Result
        let store_blob = |f: &DirEntry| {
            println!("{:?}", f.path());
            Blob::new(&f.path()).persist(&self.store).unwrap()
        };
        visit_jit_dir(&self.path, &store_blob)?;
        Ok(0)
    }

    pub fn new(path: &Path) -> Workspace {
        Workspace {
            path: path.to_path_buf(),
            store: Store::new(&path.join(".jit/objects")),
        }
    }
}

fn visit_jit_dir(path: &PathBuf, cb: &Fn(&DirEntry)) -> io::Result<()> {
    let ignore = [".", "..", ".jit", ".git", "target"];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if !ignore
            .iter()
            .any(|&x| entry.file_name().to_str().map_or(false, |v| v == x))
        {
            if path.is_dir() {
                visit_jit_dir(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
