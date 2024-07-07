use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use fs2::FileExt;

use crate::utils::create_or_get_dir;

/*
Java FFI method definition

Example:
id: hello
sig: public static native String hello(String input)
*/
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct JavaFFI {
    pub id: String,
    pub sig: String,
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FFIStore {
    #[serde(skip)]
    file_content: Option<String>,
    #[serde(skip)]
    file_path: Option<PathBuf>,
    #[serde(skip)]
    file_lock: Option<File>,
    methods: Vec<JavaFFI>,
}

impl FFIStore {
    pub fn add_ffi(&mut self, ffi: JavaFFI) {
        self.methods.retain(|i| i.id != ffi.id);
        self.methods.push(ffi);
    }

    pub fn get_all(&self) -> Vec<JavaFFI> {
        return self.methods.clone();
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.file_path.clone()
    }
}

impl FFIStore {
    pub fn save(&mut self) {
        if let (Ok(new_json), Some(file)) = (serde_json::to_string(&self), &mut self.file_lock) {
            file.set_len(0).ok();
            file.write(new_json.as_bytes()).ok();
            self.file_content = Some(new_json);
        }
    }

    pub fn from_json(json: String) -> Self {
        let mut store = serde_json::from_str::<Self>(&json).unwrap_or_default();
        store.file_content = Some(json);
        store
    }

    pub fn open_read_only(definitions_json: &Path) -> Self {
        let json = std::fs::read_to_string(&definitions_json).unwrap_or_default();
        Self::from_json(json)
    }

    pub fn read_from_file(definitions_json: &Path) -> Option<Self> {
        let json = std::fs::read_to_string(&definitions_json).unwrap_or_default();
        if let Some(dir) = definitions_json.parent() {
            create_or_get_dir(dir).ok();
        }
        let Ok(file) = File::create(definitions_json) else {
            return None;
        };
        if file.lock_exclusive().is_err() {
            return None;
        };

        let mut store = Self::from_json(json);
        store.file_lock = Some(file);
        store.file_path = Some(definitions_json.to_owned());
        Some(store)
    }
}

impl Drop for FFIStore {
    fn drop(&mut self) {
        if let Some(file) = &mut self.file_lock {
            file.unlock().ok();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{FFIStore, JavaFFI};
    use std::{fs, path::Path};

    pub fn create_test_store() -> FFIStore {
        let rand_nr: u64 = rand::random();
        FFIStore::read_from_file(
            &Path::new(".")
                .join("target")
                .join(format!("store_{rand_nr}.json")),
        )
        .expect("Should create new FFIStore")
    }

    #[test]
    pub fn should_create_new_store() {
        let store = create_test_store();
        assert!(store.file_lock.is_some());
        assert!(store.file_path.is_some());
    }

    #[test]
    pub fn should_save_to_store() {
        let mut store = create_test_store();
        store.add_ffi(JavaFFI {
            id: "test_id".to_string(),
            sig: "sig".to_string(),
        });
        let store_path = store.path().expect("Store path").clone();

        store.save();
        let json = fs::read_to_string(&store_path).expect("Read store");
        assert!(json.contains("test_id"));
        assert!(json.contains("sig"));

        drop(store);
        let json = fs::read_to_string(&store_path).expect("Read store");
        assert!(json.contains("test_id"));
        assert!(json.contains("sig"));
    }
}
