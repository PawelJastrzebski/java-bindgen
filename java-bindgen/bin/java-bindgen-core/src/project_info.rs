
#[derive(Debug)]
pub struct ProjectInfo {
    pub java_package_name: String,
    pub lib_name: String,
    pub lib_version: String,
}

impl ProjectInfo {
    pub fn set_package_name(mut self, package: &str) -> Self {
        if !package.is_empty() {
            self.java_package_name = package.to_string();
        }
        self
    }
}

impl ProjectInfo {

    pub fn jar_asymbly_name(&self) -> String {
        format!(
            "{}-{}-jar-with-dependencies.jar",
            self.lib_name, self.lib_version
        )
    }

    pub fn jar_final_name(&self) -> String {
        format!("{}-{}.jar", self.lib_name, self.lib_version)
    }
    
    pub fn get_java_class_name(&self) -> String {
        use convert_case::Casing;

        self.lib_name
            .replace("_", " ")
            .to_case(convert_case::Case::Pascal)
    }

    #[allow(dead_code)]
    pub fn get_java_method_name(&self, method_name: &str) -> String {
        let packages = self.get_packages_path();
        let class_name = self.get_java_class_name();

        // Example: Java_com_test_MyLib_hello
        // package: com.test
        // class: MyLib
        // fn: hello
        format!("Java_{}_{}_{}", packages.join("_"), class_name, method_name)
    }

    pub fn get_packages_path(&self) -> Vec<String> {
        self.java_package_name
            .to_lowercase()
            .split(".")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
}

impl From<&crate::cargo_parser::CargoToml> for ProjectInfo {
    fn from(value: &crate::cargo_parser::CargoToml) -> Self {
        let java_bindgen = value.java_bindgen().unwrap_or_default();
        ProjectInfo {
            lib_name: value.package.name.clone(),
            lib_version: value.package.version.clone(),
            java_package_name: java_bindgen.package.unwrap_or_default(),
        }
    }
}

