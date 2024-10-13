use std::process::{Command, Stdio};

fn main() {
    Command::new("node")
        .arg(".")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
}
