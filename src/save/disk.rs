use std::process::{Command, Stdio};
use std::fmt;
use inquire::{Select, error::InquireError, Confirm};
use json;
use crate::save::choose_path_folder;

#[derive(Debug, Clone)]
struct DiskPart {
    name: String,
    size: String,
    mountpoint: String,
    uuid: String,
    fsavail: String,
    path: String

}

impl fmt::Display for DiskPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name : {}, size : {}, mountpoint : {}, uuid : {}", self.name, self.size, self.mountpoint, self.uuid)
    }
}

trait Values {
    fn get_uuid(&self) -> String;
    fn get_size(&self) -> u64;
    fn get_fsavail(&self) -> u64;
}

impl Values for  DiskPart {
    fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    fn get_size(&self) -> u64 {
        self.size.parse::<u64>().unwrap()/1024
    }

    fn get_fsavail(&self) -> u64 {
        self.fsavail.parse::<u64>().unwrap()
    }
}

pub fn save_disk() {
    let whoami = Command::new("whoami").stdout(Stdio::piped()).output().expect("failed to execute process");
    let mut whoami = String::from_utf8(whoami.stdout).unwrap();
    whoami = whoami.replace("\n", "");

    if whoami != "root" {
        println!("You must run the program with sudo right to save a disk");
        std::process::exit(1);
    }

    let list_disks = Command::new("lsblk")
        .args([
            "-Jbo",
            "name,size,mountpoint,type,uuid,fsavail,path"
        ])
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");
    
    let output = String::from_utf8(list_disks.stdout).unwrap();
    let parsed = json::parse(&output).unwrap();

    let mut disks_to_display: Vec<DiskPart> = vec![];
    
    for disks in parsed["blockdevices"].members() {
        for partitions in disks["children"].members() {
            let disk_part = DiskPart {
                name: partitions["name"].to_string(),
                size: partitions["size"].to_string(),
                mountpoint: partitions["mountpoint"].to_string(),
                uuid: partitions["uuid"].to_string(),
                fsavail: partitions["fsavail"].to_string(),
                path: partitions["path"].to_string()
            };
            disks_to_display.push(disk_part);
        }
    }    

    let source: Result<DiskPart, InquireError> = Select::new("Select source", disks_to_display.clone()).prompt();
    let destination = choose_path_folder("destination");

    //TODO : a améliorer
    let disk_dest = Command::new("df")
    .args([
        "--output=avail",
        destination.as_str()
        ]) .stdout(Stdio::piped())
    .output()
    .expect("failed to execute process");
    
    let disk_dest = String::from_utf8(disk_dest.stdout).unwrap();
    let disk_dest = disk_dest.split("\n").collect::<Vec<&str>>();

    let size_destination = disk_dest[1].split_whitespace().collect::<Vec<&str>>();
    let size_destination: u64 = size_destination[0].parse().unwrap();

    match source {
        Ok(src) => {
            if src.get_size() > size_destination {
                println!("Error, destination size is smaller than source !");
                std::process::exit(1);
            } else {
                if Confirm::new(format!("Are you sure you want to save {} to {} ?", src.path, destination).as_str()).with_default(false).prompt().unwrap() {
                    Command::new("dd")
                    .args(&[
                        format!("if={}",src.path).as_str(), 
                        format!("of={}/{}_save.img", destination, src.uuid).as_str(), //choose a folder !
                        "bs=4096", 
                        "status=progress" 
                        ])
                    .status()
                    .expect("failed to execute process");
    
                    println!("saved in {destination} with the name {}_save.img", src.uuid);
                } else {
                    println!("Aborted");
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("error : {e}");
            std::process::exit(1);
        },
    }
}