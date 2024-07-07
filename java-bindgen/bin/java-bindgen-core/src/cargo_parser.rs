use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CargoToml {
    pub package: Package,
    pub lib: Option<Lib>,
}

impl CargoToml {
    pub fn java_bindgen(&self) -> Option<JavaBindgen> {
        let value = self.package.java_bindgen.as_ref()?;
        let metadata = value.get("metadata")?;

        Some(JavaBindgen {
            package: metadata
                .get("package")
                .and_then(|v| v.as_str())
                .map(|v| v.to_string()),
        })
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    #[serde(alias = "java-bindgen")]
    pub java_bindgen: Option<toml::Table>,
}

#[derive(Deserialize, Debug)]
pub struct Lib {
    #[serde(alias = "crate-type")]
    pub crate_type: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct JavaBindgen {
    pub package: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum TomlParseError {
    #[error("Cargo.toml file NotFound")]
    NotFound,
    #[error("Invalid Cargo.toml file at: {0}")]
    Invalid(PathBuf),
    #[error("Failed to open Cargo.toml file at: {0}")]
    FailedToOpen(PathBuf),
    #[error("Failed to parse Cargo.toml")]
    ParseError(#[from] toml::de::Error),
}

pub fn parse_toml(path: &Path) -> Result<CargoToml, TomlParseError> {
    let Some(full_path) = fs::canonicalize(path).ok() else {
        return Err(TomlParseError::NotFound);
    };

    if path.is_dir() {
        return Err(TomlParseError::Invalid(full_path));
    }

    let Ok(toml_content) = std::fs::read_to_string(&path) else {
        return Err(TomlParseError::FailedToOpen(full_path));
    };

    // let value: toml::Value = toml::from_str(&toml_content).map_err(TomlParseError::ParseError)?;
    // dbg!(value);

    toml::from_str(&toml_content).map_err(TomlParseError::ParseError)
}

#[cfg(test)]
pub mod tests {
    use std::path::Path;

    #[test]
    pub fn should_read_from_toml_file() {
        let paht = Path::new("tests_assets/example.toml");
        let file = super::parse_toml(&paht).unwrap();
        let java_bindgen = file.java_bindgen().unwrap_or_default();
        assert_eq!("mylib", file.package.name);
        assert_eq!("0.1.1", file.package.version);
        assert_eq!("com.test", java_bindgen.package.unwrap_or_default());
        assert_eq!(vec!["cdylib"], file.lib.unwrap().crate_type.unwrap());
    }
}
