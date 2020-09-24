use std::{
    ffi::CString,
    fs,
    io::{self, Read},
    path::PathBuf,
};

pub mod shaders;
pub mod textures;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct Loader {
    root_path: PathBuf,
}
impl Loader {
    pub fn new(content_path: &'static str) -> Loader {
        let path = std::env::current_exe().unwrap();
        let path_parent = path.parent().unwrap();
        let root_path = path_parent.join("res/").join(content_path);

        Loader { root_path }
    }

    pub fn load_as_cstring(&self, asset_path: &'static str) -> CString {
        let mut file = fs::File::open(self.root_path.join(asset_path)).unwrap();
        let mut buffer = Vec::with_capacity(file.metadata().unwrap().len() as usize + 1);

        file.read_to_end(&mut buffer);
        if buffer.iter().find(|i| **i == 0).is_some() {
            panic!(Error::FileContainsNil);
        }

        unsafe { CString::from_vec_unchecked(buffer) }
    }

    pub fn load_as_image(&self, asset_path: &'static str) -> image::DynamicImage {
        image::open(self.root_path.join(asset_path)).unwrap()
    }
}
