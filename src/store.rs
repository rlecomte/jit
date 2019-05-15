use deflate::write::ZlibEncoder;
use deflate::Compression;
use hex;
use ring::digest;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct Blob {
    pub path: PathBuf,
}

impl Blob {
    pub fn new(path: &Path) -> Blob {
        Blob {
            path: path.to_path_buf(),
        }
    }
}

pub struct Store {
    pub path: PathBuf,
}

impl Store {
    pub fn new(path: &Path) -> Store {
        Store {
            path: path.to_path_buf(),
        }
    }
}

pub trait StoreObject {
    fn persist(&self, store: &Store) -> io::Result<()>;
}

impl StoreObject for Blob {
    fn persist(&self, store: &Store) -> io::Result<()> {
        // create digest
        let mut ctx = digest::Context::new(&digest::SHA1);
        ctx.update(b"blob ");
        ctx.update(format!("{}", self.path.metadata()?.len()).as_bytes());
        ctx.update(b"\0");

        {
            let input = fs::File::open(&self.path)?;
            let mut reader = io::BufReader::new(input);
            let mut buffer = [0; 1024];

            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                ctx.update(&buffer[..count]);
            }
        }

        // add file content into digest
        let oid: String = hex::encode(ctx.finish().as_ref());

        // create commit directory from first 2 char of oid
        let (d, f) = oid.split_at(2);
        let dir = store.path.join(d);
        let file = dir.join(f);
        fs::create_dir_all(&dir)?;

        // create and write blob into temporary file
        let temppath = dir.join(Uuid::new_v4().to_string());
        let tempfile = fs::File::create(&temppath)?;

        let mut encoder = ZlibEncoder::new(tempfile, Compression::Default);
        encoder.write_all(b"blob ")?;
        encoder.write_all(format!("{}", self.path.metadata()?.len()).as_bytes())?;
        encoder.write_all(b"\0")?;

        {
            let input = fs::File::open(&self.path)?;
            let mut reader = io::BufReader::new(input);
            let mut buffer = [0; 1024];

            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                encoder.write_all(&buffer[..count])?;
            }
        }

        encoder.finish()?;
        fs::rename(temppath, file)?;

        Ok(())
    }
}
