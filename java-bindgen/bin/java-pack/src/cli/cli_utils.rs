use std::path::{Path, PathBuf};

use color_eyre::eyre::Context;
use java_bindgen_core::utils::create_or_get_dir;

pub fn create_file(
    directory: &Path,
    file_name: &str,
    content: &str,
) -> color_eyre::Result<PathBuf> {
    let file_path = directory.join(file_name);
    std::fs::write(&file_path, content).wrap_err(format!("Failed to create {file_name} file "))?;
    Ok(file_path)
}

pub fn exec_command_silent(dir: &Path, command: &str) -> (i32, String, String) {
    let dir_path = dir.to_string_lossy();

    if !dir.exists() || !dir.is_dir() {
        // Skip executon for invalid Paths
        return (
            -1,
            "".to_string(),
            format!("Directory not found: {}", dir_path),
        );
    }
    let (code, out, err) = shells::sh!("cd {} && {}", dir_path, command);
    (code, out, err)
}

#[allow(dead_code)]
pub static COLOR_BALCK: ansi_term::Colour = ansi_term::Colour::Black;
#[allow(dead_code)]
pub static COLOR_YELLOW: ansi_term::Colour = ansi_term::Colour::Yellow;
pub static COLOR_GREEN: ansi_term::Colour = ansi_term::Colour::Green;
pub static COLOR_RED: ansi_term::Colour = ansi_term::Colour::Red;
pub static COLOR_WHITE: ansi_term::Colour = ansi_term::Colour::White;
pub static COLOR_WHITE_RGB: ansi_term::Colour = ansi_term::Colour::RGB(250, 250, 250);
pub static COLOR_GRAY: ansi_term::Colour = ansi_term::Colour::RGB(140, 140, 140);

pub fn icon(kind: &str) -> String {
    match kind {
        "yellow" | "warn" => COLOR_WHITE_RGB
            .on(COLOR_GRAY)
            .bold()
            .paint(" ! ")
            .to_string(),
        "red" | "error" => COLOR_WHITE_RGB
            .on(COLOR_RED)
            .bold()
            .paint(" ✕ ")
            .to_string(),
        "green" | "ok" | _ => COLOR_WHITE_RGB
            .on(COLOR_GREEN)
            .bold()
            .paint(" ✓ ")
            .to_string(),
    }
}

pub fn print_option(label: &str, version: Option<&String>, required: bool) {
    let label = flabel(label);
    if let Some(ref version) = version {
        let version = strip_ansi_escapes::strip_str(&version);
        println!("{} {label}{}", icon("ok"), COLOR_GREEN.paint(version))
    } else if !required {
        println!(
            "{} {label}{}",
            icon("warn"),
            COLOR_WHITE.dimmed().paint("Not found")
        )
    } else {
        println!("{} {label}{}", icon("red"), COLOR_RED.paint("Not found"))
    }
}

pub fn flabel(label: &str) -> String {
    use pad::PadStr;
    let label = if label.is_empty() {
        label.to_string()
    } else {
        format!("{}", label)
    };

    format!("{}", COLOR_WHITE.dimmed().bold().paint(label)).pad_to_width(40)
}

pub fn header(label: &str) -> String {
    let size = crossterm::terminal::size().unwrap_or((0, 0));
    let width = size.0 as usize;

    let right_witdth = (width as f32 / 1.8) as usize - label.len() - 3 - 2;
    format!(
        "{} {} {}\n",
        "═".repeat(right_witdth / 2),
        COLOR_WHITE.bold().dimmed().paint(label),
        "═".repeat(right_witdth / 2)
    )
}

pub fn exit_msg(label: &str) -> String {
    let size = crossterm::terminal::size().unwrap_or((0, 0));
    let width = size.0 as usize;

    let right_witdth = (width as f32 / 1.8) as usize - label.len() - 3 - 2;
    format!(
        "{} {} {}\n",
        COLOR_RED.paint("═".repeat(right_witdth / 2)),
        COLOR_RED.bold().paint(label),
        COLOR_RED.paint("═".repeat(right_witdth / 2))
    )
}

pub fn ready_info(is_ready: bool, label: &str) -> String {
    if is_ready {
        COLOR_WHITE_RGB
            .on(COLOR_GREEN)
            .bold()
            .paint(format!("  {label} ✓  "))
            .to_string()
    } else {
        COLOR_WHITE_RGB
            .on(COLOR_RED)
            .bold()
            .paint(format!("  {label} ✕  "))
            .to_string()
    }
}

pub fn exec_command(directory: &Path, command: &str, info: &str) -> color_eyre::Result<()> {
    let dir = create_or_get_dir(directory)?;
    let dir_path = dir.to_string_lossy().to_string();

    {
        println!("{}", header(info));
        let command_label = COLOR_WHITE.dimmed().paint("Run: ");
        let command = COLOR_WHITE_RGB.bold().paint(command);
        let dri = COLOR_WHITE.dimmed().underline().paint(&dir_path);
        println!("{command_label} {command}\n{dri}\n");
    }

    // Spawn process

    let command = command.replace("\n", " ").replace("\t", "");
    let mut process = subprocess::Exec::shell(&format!("cd {} && {}", dir_path, command))
        .stdout(subprocess::Redirection::Pipe)
        .stderr(subprocess::Redirection::Merge)
        .popen()?;

    // Read output
    while process.poll().is_none() {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let (out, _) = process.communicate(None)?;
        if let Some(out) = out.as_ref() {
            println!("{}", out)
        }
    }

    let status_code = process.poll().expect("Status code");
    println!("{}\n", ready_info(status_code.success(), "OK"));

    if !status_code.success() {
        exit()
    }

    Ok(())
}

pub fn exit() {
    println!("{}", COLOR_RED.paint(exit_msg("Exit")));
    std::process::exit(-1)
}

pub fn sleep(milis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milis));
}