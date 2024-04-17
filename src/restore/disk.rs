use crate::{utils, commands, r#struct::DiskPart};
use inquire::{Select, error::InquireError, Confirm, ui::{Color, RenderConfig, StyleSheet}};



pub fn restore_disk() {
    let whoami = commands::whoami();

    if whoami != "root" {
        println!("You must run the program with sudo rights to save a disk");
        std::process::exit(1);
    }

    println!("Choose an image to restore");
    let image_disk = utils::choose_image_disk();

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

    let destination: Result<DiskPart, InquireError> = Select::new("Select disk to restore", disks_to_display.clone()).prompt();

    match destination {
        Ok(dest) => {

            let size_image_disk = utils::calculate_file_size(image_disk.as_str());

            if size_image_disk > dest.get_size("byte") {
                println!("\x1b[31mError, destination size is smaller than source !\x1b[0m");
                std::process::exit(1);
            }

            let mut render_config = RenderConfig::default();
            render_config.prompt = StyleSheet::new().with_fg(Color::DarkYellow);

            if Confirm::new(format!("Are you sure you want to restore {} to {} ?", image_disk, dest.path).as_str()).with_render_config(render_config).with_default(false).prompt().unwrap() {

                let s1 = format!("if={}",image_disk);
                let s2 = format!("of={}", dest.path);
                let args: Vec<&str> = vec![s1.as_str(), s2.as_str(),"bs=4096", "status=progress"];

                match commands::dd(args) {
                    Ok(_) => {
                        println!("\x1b[32mRestoration completed successfully\x1b[0m");
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
        Err(e) => {
            println!("\x1b[31mError : {e}\x1b[0m");
            std::process::exit(1);
        }
    }
}