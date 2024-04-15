use std::process::{Command, Stdio};
use std::fmt;
use inquire::{Select, error::InquireError, Confirm};
use json;
use crate::save::{choose_path_folder, get_available_space_disk};

#[derive(Debug, Clone)]
struct DiskPart {
    name: String,
    size: String,
    mountpoint: String,
    uuid: String,
    path: String

}

impl fmt::Display for DiskPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name : {}, size : {}, mountpoint : {}, uuid : {}", self.name, self.size, self.mountpoint, self.uuid)
    }
}

impl DiskPart {
    fn get_size(&self) -> u64 {
        self.size.parse::<u64>().unwrap()/1024
    }
}

pub fn save_disk() {
    let whoami = Command::new("whoami").stdout(Stdio::piped()).output().expect("failed to execute process");
    let mut whoami = String::from_utf8(whoami.stdout).unwrap();
    whoami = whoami.replace("\n", "");

    if whoami != "root" {
        println!("You must run the program with sudo rights to save a disk");
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
                path: partitions["path"].to_string()
            };
            disks_to_display.push(disk_part);
        }
    }    

    let source: Result<DiskPart, InquireError> = Select::new("Select source", disks_to_display.clone()).prompt();
    let destination = choose_path_folder("destination");
    let size_available = get_available_space_disk(destination.as_str());

    if let Err(e) = size_available {
        println!("Error {e}");
        std::process::exit(1);
    }

    match source {
        Ok(src) => {
            if Ok(src.get_size()) > size_available {
                println!("Error, destination size is smaller than source !");
                std::process::exit(1);
            } else {
                if Confirm::new(format!("Are you sure you want to save {} to {} ?", src.path, destination).as_str()).with_default(false).prompt().unwrap() {
                    match Command::new("dd")
                    .args(&[
                        format!("if={}",src.path).as_str(), 
                        format!("of={}/{}_save.img", destination, src.uuid).as_str(),
                        "bs=4096", 
                        "status=progress" 
                        ])
                    .status() {
                        Ok(_) => {
                            println!("Backup completed successfully");
                            println!("saved in {destination} with the name {}_save.img", src.uuid);
                        }
                        Err(e) => {
                            println!("Error : {e}");
                        }
                    }
                } else {
                    println!("Aborted");
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("Error : {e}");
            std::process::exit(1);
        },
    }
}