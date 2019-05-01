use std::fs::{self, DirBuilder, DirEntry};
use std::io;
use std::path::Path;

pub(crate) struct Workspace<'a> {
    pub path: &'a Path,
}

impl<'a> Workspace<'a> {
    pub fn initialize(&'a self) -> io::Result<i32> {
        let p = self.path.canonicalize()?;
        println!("Initialize empty jit repo in {}", p.display());

        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder.create(p.join(".jit/objects"))?;
        builder.create(p.join(".jit/refs"))?;
        Ok(0)
    }

    pub fn commit(&'a self) -> io::Result<i32> {
        let show_path = |f: &DirEntry| println!("{:?}", f.path());
        visit_jit_dir(self.path, &show_path)?;
        Ok(0)
    }

    pub fn new(path: &'a str) -> Workspace {
        Workspace {
            path: Path::new(path),
        }
    }
}

fn visit_jit_dir(path: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    let ignore = [".", "..", ".jit"];

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
