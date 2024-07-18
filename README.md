<div align="center">
<h1 style='font-size: 1.8rem'>
☕ + 🦀 = ❤️‍🔥
</h1>
<h3 style='font-size: 1.6rem'>Java JNI Bindings Generator</h3>
</div>

## Introduction 👋
Welcome to java-bindgen, an easy-to-use Java JNI (Java Native Interface) bindings generator and CLI tool for building Java JARs. This tool simplifies the process of integrating Rust and Java, enabling you to leverage Rust's performance and safety in your Java applications.

## Goal 🚀 
Develop a robust and safe framework that facilitates seamless and secure integration between Java and Rust using JNI, minimizing the complexity and risks associated with native code interoperability.

## Features 🎖️
- Convenient error handling using `JResult<T, JException>` with propagation to the Java layer.
- Automatic type conversion for Java primitives like `String`, `byte[]`, `int`, `long`, `float`, `boolean`, etc.
- Custom types with `#[derive(IntoRust, IntoJava)]` for seamless integration.
- Integrated Logger `#[derive(JLogger)]` for better debugging and logging support.
- Rust error `stack trace` attached to Java Exceptions for improved error diagnostics.

<br />
<br />

## Rust Project Setup 🦀
Install `java-pack` CLI 🛠️
```sh
cargo install java-pack
```
Add  `java-bindgen` dependency
```sh
cargo add java-bindgen
```
Add `Cargo.toml` configuration:
```toml
[package.java-bindgen.metadata]
package = "your.java.package"
```
Set `crate-type`:

```toml
[lib]
crate-type = ["cdylib"]
```
### Verify Your Configuration 🔦
To confirm your setup, run the following command:
```sh
java-pack info
```
<br />
<br />

## Example

☢️ `The following examples do not compile due to missing configurations in the Cargo.toml file.` ☢️

lib.rs
```rust compile_fail
use java_bindgen::prelude::*;

#[derive(IntoJava, Default)]
struct User {
    name: String,
}

#[java_bindgen]
fn getUser() -> JResult<User> {
    Ok(User {
        name: "Tom".to_string(),
    })
}
```

Building jar 🫙
```sh
java-pack build
```

Produces the following Java interface and User class:
```java
public static native User getUser();
```

```java
@ToString
@Getter
@Setter
@AllArgsConstructor
public class User {
	String name;
}
```

<br />

## Testing 💯
Create Java test project:
```sh
java-pack new-test
```
Add Test:
```java
@Test
public void should_get_user() {
    UserClass user = TestMacro.getUser();
    assertEquals("Tom", user.getName());
}
```

Run tests:
```sh
java-pack test
```

<br />
<br />

# Project 📦

#### Project structure 📌

- `java-bindgen` - main crate
- `java-bindgen-macro` - macro system
- `java-bindgen-core` - shared lib
- `java-pack` - building tool


#### Project status 🚧
Project is in early state of development. Each release is prior tested but api changes will most likely to happen in the future as the project progress.

#### Roadmap 📆
To be determined. If you like the project, please consider giving it a ⭐, filing an issue ❗, or submitting a pull request (PR) ✅. Your feedback and contributions are highly appreciated! 

- [GitHub](https://github.com/PawelJastrzebski/java-bindgen)

<br />

## More Examples 🤖

#### Logger
lib.rs
```rust compile_fail
use java_bindgen::prelude::*;

#[derive(JLogger)]
struct Log();

#[java_bindgen]
fn test_logger<'a>(env: &mut JNIEnv<'a>, name: String) -> JResult<()> {
    let logger = Log::init(env);
    let msg = format!("Hello {name}, Welcome to Rust!");
    logger.info(msg, env);
    Ok(())
}
```
output
```sh
[main] INFO  com.test.macro.TestMacro  - Hello Java Bindgen, Welcome to Rust!
```
<br />

## Full Examples
For full examples visit: 
[github.com/java-bindgen/examples](https://github.com/PawelJastrzebski/java-bindgen/tree/main/examples)