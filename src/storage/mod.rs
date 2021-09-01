mod local_storage;
pub use local_storage::LocalStorage;

use glob;
use std::fmt;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{ Path, PathBuf };

pub trait Storage
where
    Self::Item: ToPathBuf,
    Self::Items: Iterator<Item=Self::Item>
{
    type Items;
    type Item;
    
    fn glob(&self, pattern: &str) -> Result<Self::Items, PatternError>;
    fn exists<P: AsRef<Path>>(&self, item: P) -> bool;
    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()>;
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<()>;
}

pub trait ToPathBuf {
    fn to_path(self) -> Result<PathBuf, ToPathBufError>;
}

pub fn search<S: Storage>(storage: &S, pattern: &str) -> Result<impl Iterator<Item=PathBuf>, PatternError> {
    let items = storage.glob(pattern)?;
    let paths = items.flat_map(|item| item.to_path());
    Ok(paths)
}

#[derive(Debug)]
pub struct PatternError {
    msg: &'static str 
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pattern error: {}", self.msg)
    }
}

impl Error for PatternError {}

#[derive(Debug)]
pub struct ToPathBufError {}

impl fmt::Display for ToPathBufError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "path error")
    }
}

impl Error for ToPathBufError {}
