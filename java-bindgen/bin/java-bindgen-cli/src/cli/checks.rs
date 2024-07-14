use std::path::Path;

use java_bindgen_core::cargo_parser::{parse_toml, CargoTomlFile, JavaBindgen, Lib};

use super::cli_utils::{self, flabel, header, icon, print_option, ready_info, COLOR_GREEN, COLOR_RED};

pub struct SystemSetupStatus {
    pub java_version: Option<String>,
    pub cargo_version: Option<String>,
    pub mvn_version: Option<String>,
    pub gradle_version: Option<String>,
}

impl SystemSetupStatus {
    pub fn pretty_print(&self) {
        print_option("Java", self.java_version.as_ref(), true);
        print_option("Maven", self.mvn_version.as_ref(), true);
        print_option(
            "Cargo",
            self.cargo_version.as_ref(),
            self.gradle_version.is_none(),
        );
        print_option(
            "Gradle",
            self.gradle_version.as_ref(),
            self.cargo_version.is_none(),
        );
    }

    pub fn is_ready(&self) -> bool {
        let java_and_rust = self.java_version.is_some() && self.cargo_version.is_some();
        let mvn_or_gradle = self.mvn_version.is_some() || self.gradle_version.is_some();
        java_and_rust && mvn_or_gradle
    }

    pub fn get_status(&self) -> String {
        ready_info(self.is_ready(), "Ready")
    }

    fn check_java(dir: &Path) -> Option<String> {
        let (code, out, _err) = cli_utils::exec_command_silent(&dir, "java --version");
        if code != 0 {
            return None;
        }
        if let Some(line) = out.lines().into_iter().next() {
            return Some(line.to_string());
        }
        None
    }

    fn check_maven(dir: &Path) -> Option<String> {
        let (code, out, _err) = cli_utils::exec_command_silent(&dir, "mvn --version");
        if code != 0 {
            return None;
        }
        if let Some(line) = out.lines().into_iter().next() {
            return Some(line.to_string());
        }
        None
    }

    fn check_cargo(dir: &Path) -> Option<String> {
        let (code, out, _err) = cli_utils::exec_command_silent(&dir, "cargo --version");
        if code != 0 {
            return None;
        }
        if let Some(line) = out.lines().into_iter().next() {
            return Some(line.to_string());
        }
        None
    }

    fn check_gradle(dir: &Path) -> Option<String> {
        let (code, out, _err) = cli_utils::exec_command_silent(&dir, "gradle --version");
        if code != 0 {
            return None;
        }
        if let Some(line) = out.lines().into_iter().next() {
            return Some(line.to_string());
        }
        None
    }

    pub fn check(dir: &Path) -> SystemSetupStatus {
        SystemSetupStatus {
            java_version: SystemSetupStatus::check_java(&dir),
            mvn_version: SystemSetupStatus::check_maven(&dir),
            cargo_version: SystemSetupStatus::check_cargo(&dir),
            gradle_version: SystemSetupStatus::check_gradle(&dir),
        }
    }
}

pub struct CargoSetupStatus {
    cargo_toml_present: bool,
    cargo_lib_setup: bool,
    cargo_java_bindgen_setup: bool,
}

impl CargoSetupStatus {
    pub fn check_lib_setup(lib: Option<&Lib>) -> bool {
        if let Some(lib) = lib {
            if let Some(ref crate_type) = lib.crate_type {
                return crate_type.contains(&"cdylib".to_string());
            }
        }

        false
    }

    fn check_java_bindgen(java_bindgen: Option<JavaBindgen>) -> bool {
        if let Some(java_bindgen) = java_bindgen {
            if let Some(ref package) = java_bindgen.package {
                // todo validate java package name
                return package.len() > 0;
            }
        }
        false
    }

    pub fn check(dir: &Path) -> Self {
        let toml_path = dir.join("Cargo.toml");
        if let Some(CargoTomlFile { toml_parsed, .. }) = parse_toml(&toml_path).ok() {
            return CargoSetupStatus {
                cargo_toml_present: true,
                cargo_lib_setup: Self::check_lib_setup(toml_parsed.lib.as_ref()),
                cargo_java_bindgen_setup: Self::check_java_bindgen(toml_parsed.java_bindgen()),
            };
        }

        CargoSetupStatus {
            cargo_toml_present: false,
            cargo_lib_setup: false,
            cargo_java_bindgen_setup: false,
        }
    }

    pub fn pretty_print(&self) {
        let cargo_label = flabel("Cargo.toml");
        let lib_label = flabel("[lib]");
        let bindgen_label = flabel("[java-bindgen]");
        if !self.cargo_toml_present {
            println!(
                "{} {cargo_label}{}",
                icon("red"),
                COLOR_RED.paint("Not found")
            );
            return;
        } else {
            println!("{} {cargo_label}{}", icon("ok"), COLOR_GREEN.paint("Ok"));
        }

        if !self.cargo_lib_setup {
            println!(
                "{} {lib_label}{}",
                icon("red"),
                COLOR_RED.paint("Invalid Configuration")
            );
        } else {
            println!("{} {lib_label}{}", icon("ok"), COLOR_GREEN.paint("Ok"));
        }

        if !self.cargo_java_bindgen_setup {
            println!(
                "{} {bindgen_label}{}",
                icon("red"),
                COLOR_RED.paint("Invalid Configuration")
            );
        } else {
            println!("{} {bindgen_label}{}", icon("ok"), COLOR_GREEN.paint("Ok"));
        }
    }

    pub fn is_ready(&self) -> bool {
        self.cargo_toml_present && self.cargo_lib_setup && self.cargo_java_bindgen_setup
    }

    pub fn get_status(&self) -> String {
        ready_info(self.is_ready(), "Ready")
    }
}

pub struct CheckResult {
    system: SystemSetupStatus,
    cargo: CargoSetupStatus,
}

impl CheckResult {
    pub fn check(dir: &Path) -> Self {
        Self {
            system: SystemSetupStatus::check(dir),
            cargo: CargoSetupStatus::check(dir),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.system.is_ready() && self.cargo.is_ready()
    }

    pub fn print_status(&self) {
        println!("{}", header("System"));
        self.system.pretty_print();
        println!("{}    {}", flabel(""), self.system.get_status());

        println!("");

        println!("{}", header("Rust Project"));
        self.cargo.pretty_print();
        println!("{}    {}", flabel(""), self.cargo.get_status());
        println!("");
    }
}
