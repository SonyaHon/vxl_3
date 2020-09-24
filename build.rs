extern crate walkdir;

use gl_generator::{Api, Fallbacks, Profile, Registry};
use std::fs::{self, DirBuilder, File};
use std::path::PathBuf;
use std::{env, path::Path};
use walkdir::WalkDir;

fn main() {
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let executable_path = locate_target_dir_from_output_dir(&dest)
        .expect("failed to find target dir")
        .join(env::var("PROFILE").unwrap());

    std::fs::remove_dir_all(&executable_path.join("res/")).unwrap();
    copy(&manifest_dir.join("res/"), &executable_path.join("res/"));

    println!("cargo:rerun-if-changed=build.rs");

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();
    Registry::new(Api::Gles2, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(gl_generator::StructGenerator, &mut file)
        .unwrap();
}

fn locate_target_dir_from_output_dir(mut target_dir_search: &Path) -> Option<&Path> {
    loop {
        // if path ends with "target", we assume this is correct dir
        if target_dir_search.ends_with("target") {
            return Some(target_dir_search);
        }

        // otherwise, keep going up in tree until we find "target" dir
        target_dir_search = match target_dir_search.parent() {
            Some(path) => path,
            None => break,
        }
    }

    None
}

fn copy(from: &Path, to: &Path) {
    let from_path: PathBuf = from.into();
    let to_path: PathBuf = to.into();
    for entry in WalkDir::new(from_path.clone()) {
        let entry = entry.unwrap();

        if let Ok(rel_path) = entry.path().strip_prefix(&from_path) {
            let target_path = to_path.join(rel_path);

            if entry.file_type().is_dir() {
                DirBuilder::new()
                    .recursive(true)
                    .create(target_path)
                    .expect("failed to create target dir");
            } else {
                fs::copy(entry.path(), &target_path).expect("failed to copy");
            }
        }
    }
}
