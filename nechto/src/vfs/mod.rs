use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("read error: {path_spec}: {error}")]
    Read {
        path_spec: String,
        error: std::io::Error,
    },

    #[error("Ill-formed path spec: {0}")]
    IllFormedPathSpec(String),

    #[error("Path prefix not found: {0}")]
    PathPrefixNotFound(String),

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct VirtualFs {
    search_paths: HashMap<String, PathBuf>,
}

impl VirtualFs {
    pub fn new() -> Self {
        Self {
            search_paths: HashMap::new(),
        }
    }

    pub fn add_search_path(&mut self, prefix: impl Into<String>, path: PathBuf) {
        self.search_paths.insert(prefix.into(), path);
    }

    fn resolve_path_spec(&self, path: impl IntoPathSpec) -> Result<PathBuf, Error> {
        let path_spec = path.as_path_spec()?;

        let (prefix, relative_path) = path_spec.split();
        let search_path = self
            .search_paths
            .get(prefix)
            .ok_or_else(|| Error::PathPrefixNotFound(prefix.to_string()))?;

        Ok(search_path.join(relative_path))
    }

    pub fn exists(&self, path: impl IntoPathSpec) -> Result<bool, Error> {
        let path = self.resolve_path_spec(path)?;

        Ok(path.try_exists()?)
    }

    pub fn read(&self, path: impl IntoPathSpec) -> Result<Vec<u8>, Error> {
        let path_spec = path.as_path_spec()?;

        let path = self.resolve_path_spec(path_spec)?;

        let data = std::fs::read(&path).map_err(|error| Error::Read {
            path_spec: path_spec.to_string(),
            error,
        })?;

        Ok(data)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PathSpec<'a>(&'a str);

impl PathSpec<'_> {
    pub const PREFIX_SEPARATOR: char = '/';

    pub fn split(&self) -> (&str, &str) {
        self.0.split_once(Self::PREFIX_SEPARATOR).unwrap()
    }
}

pub trait IntoPathSpec {
    fn as_path_spec(&self) -> Result<PathSpec, Error>;
}

impl IntoPathSpec for String {
    fn as_path_spec(&self) -> Result<PathSpec, Error> {
        self.split_once(PathSpec::PREFIX_SEPARATOR)
            .ok_or_else(|| Error::IllFormedPathSpec(self.clone()))?;
        Ok(PathSpec(self.as_ref()))
    }
}

impl IntoPathSpec for &'_ str {
    fn as_path_spec(&self) -> Result<PathSpec, Error> {
        self.split_once(PathSpec::PREFIX_SEPARATOR)
            .ok_or_else(|| Error::IllFormedPathSpec(self.to_string()))?;
        Ok(PathSpec(self))
    }
}

impl<'a> IntoPathSpec for PathSpec<'a> {
    fn as_path_spec(&self) -> Result<PathSpec, Error> {
        Ok(self.clone())
    }
}

impl From<PathSpec<'_>> for String {
    fn from(value: PathSpec) -> Self {
        value.0.to_string()
    }
}

impl<'a> From<PathSpec<'a>> for &'a str {
    fn from(value: PathSpec<'a>) -> Self {
        value.0
    }
}

impl std::fmt::Display for PathSpec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
