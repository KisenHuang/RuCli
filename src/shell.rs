use std::process::Command;
use execute::Execute;

pub fn bash(cmd: &str) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    println!("{}", out);
}