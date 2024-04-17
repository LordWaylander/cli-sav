use std::fmt;
use inquire::{Select, error::InquireError, Confirm, ui::{Color, RenderConfig, StyleSheet}};
use json;
use crate::{utils, commands};

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
        write!(f, "name : {}, size : {}G, mountpoint : {}, uuid : {}", self.name, self.get_size("giga-byte"), self.mountpoint, self.uuid)
    }
}

impl DiskPart {
    fn get_size(&self, unit: &str) -> u64 {
        match unit {
            "byte" => {
                self.size.parse::<u64>().unwrap()
            }
            "giga-byte" => {
                self.size.parse::<u64>().unwrap()/1073741824
            }
            _ => {
                todo!("only B and GB available");
            }
        }
         //return in bytes
    }
}

pub fn save_disk() {
    let whoami = commands::whoami();

    if whoami != "root" {
        println!("You must run the program with sudo rights to save a disk");
        std::process::exit(1);
    }

    let args: Vec<&str> = vec!["-Jbo", "name,size,mountpoint,type,uuid,fsavail,path"];
    let disks = commands::get_list_disk(args);
    let parsed = json::parse(&disks).unwrap();

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
    
    match source {
        Ok(src) => {
            println!("Choose the destination");
            let destination = utils::choose_path_folder("destination");
            let size_available = utils::get_available_space_disk(destination.as_str());

            if src.get_size("byte") > size_available {
                println!("\x1b[31mError, destination size is smaller than source !\x1b[0m");
                std::process::exit(1);
            } else {
                let mut render_config = RenderConfig::default();
                render_config.prompt = StyleSheet::new().with_fg(Color::DarkYellow);

                if Confirm::new(format!("Are you sure you want to save {} to {} ?", src.path, destination).as_str()).with_render_config(render_config).with_default(false).prompt().unwrap() {

                    let s1 = format!("if={}",src.path);
                    let s2 = format!("of={}/{}_save.img", destination, src.uuid);
                    let args: Vec<&str> = vec![s1.as_str(), s2.as_str(),"bs=4096", "status=progress"];

                    match commands::dd(args) {
                        Ok(_) => {
                            println!("\x1b[32mBackup completed successfully\x1b[0m");
                            println!("\x1b[32msaved in {destination} with the name {}_save.img\x1b[0m", src.uuid);
                        }
                        Err(e) => {
                            println!("\x1b[31mError : {}\x1b[0m", e.kind());
                        }
                    }
                } else {
                    println!("\x1b[33mAborted by user\x1b[0m");
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("\x1b[31mError : {e}\x1b[0m");
            std::process::exit(1);
        },
    }
}