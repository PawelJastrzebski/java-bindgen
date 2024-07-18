pub mod build {
    pub static JAVA_LIB_TEMPLATE: &str = include_str!("./build/Lib.java.template");
    pub static JAVA_CLASS_TEMPLATE: &str = include_str!("./build/Class.java.template");
    pub static POM_TEMPLATE: &str = include_str!("./build/pom.xml.template");
}

pub mod test {
    pub static JAVA_TEST_TEMPLATE: &str = include_str!("./test/Test.java.template");
    pub static JAVA_TEST_POM_TEMPLATE: &str = include_str!("./test/pom.xml.template");
    pub static JAVA_TEST_LOG4J_PROPERTIES: &str = include_str!("./test/log4j.properties.template");
    pub static JAVA_TEST_GIT_IGNORE: &str = include_str!("./test/.gitignore.template");
}

