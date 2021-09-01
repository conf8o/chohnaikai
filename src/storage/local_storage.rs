use super::*;

// Implementation of Storage

impl ToPathBuf for glob::GlobResult {
    fn to_path(self) -> Result<PathBuf, ToPathBufError> {
        match self {
            Ok(path) => Ok(path),
            Err(_) => Err(ToPathBufError {})
        }
    }
}

pub struct LocalStorage {}

impl Storage for LocalStorage {
    type Items = glob::Paths;
    type Item = glob::GlobResult;

    fn glob(&self, pattern: &str) -> Result<glob::Paths, PatternError> {
        match glob::glob(pattern) {
            Err(glob::PatternError { pos: _, msg }) => {
                Err(PatternError { msg: msg })
            },
            Ok(paths) => Ok(paths)
        }
    }

    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<()> {
        fs::rename(from, to)
    }
}
