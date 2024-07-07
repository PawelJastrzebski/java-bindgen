use std::path::{Path, PathBuf};


pub fn cargo_toml_path(project_dir: &Path) -> PathBuf {
    project_dir.join("Cargo.toml")
}

pub fn java_build_dir(project_dir: &Path) -> PathBuf {
    project_dir.join("target").join("java_bindgen")
}

pub fn ffi_definitions_path(project_dir: &Path) -> PathBuf {
    java_build_dir(project_dir).join("java_ffi.json")
}