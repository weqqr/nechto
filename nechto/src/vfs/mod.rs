use std::collections::HashMap;
use std::path::PathBuf;

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

    pub fn read(&self, path: impl IntoPathSpec) -> Vec<u8> {
        let path_spec = path.as_path_spec();

        let (prefix, relative_path) = path_spec.split();
        let search_path = self.search_paths.get(prefix).unwrap();

        std::fs::read(search_path.join(relative_path)).unwrap()
    }
}

pub struct PathSpec<'a>(&'a str);

impl PathSpec<'_> {
    pub const PREFIX_SEPARATOR: char = '/';

    pub fn split(&self) -> (&str, &str) {
        self.0.split_once(Self::PREFIX_SEPARATOR).unwrap()
    }
}

pub trait IntoPathSpec {
    fn as_path_spec(&self) -> PathSpec;
}

impl IntoPathSpec for String {
    fn as_path_spec(&self) -> PathSpec {
        PathSpec(self.as_ref())
    }
}

impl IntoPathSpec for &'_ str {
    fn as_path_spec(&self) -> PathSpec {
        PathSpec(self)
    }
}
