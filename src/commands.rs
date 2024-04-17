use std::process::{Command, Stdio, ExitStatus};

pub fn whoami() -> String {
    let whoami = Command::new("whoami").stdout(Stdio::piped()).output().expect("failed to execute process");
    let mut whoami = String::from_utf8(whoami.stdout).unwrap();
    whoami = whoami.replace("\n", "");

    return whoami;
}

pub fn get_list_disk(args: Vec<&str>) -> String {
    let list_disks = Command::new("lsblk")
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");
    
    let list_disks_string = String::from_utf8(list_disks.stdout).unwrap();

    return list_disks_string;
}

pub fn dd(args: Vec<&str>) -> Result<ExitStatus, std::io::Error> {
    Command::new("dd")
    .args(args)
    .status()
}

pub fn rsync(args: Vec<&str>) -> Result<ExitStatus, std::io::Error> {
    Command::new("rsync").args(args).status()
}