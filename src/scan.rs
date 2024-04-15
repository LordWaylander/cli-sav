use std::process::Command;

pub fn main() {
    Command::new("lsblk").status().expect("failed to execute process");
}