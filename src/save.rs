use std::process::{Command, Stdio};
use clap::Parser;
use inquire::{Select, error::InquireError, Confirm};
use std::fs;

mod disk;
mod folder;
mod files;

/// Save a Disk, Folder or Files, 
/// option required : "disk" (sudo rights necessary), "folder" or "files"

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(index = 1, required = true)]
    /// disk, folder, files
    pub type_to_save: String,
}

pub fn main(options: Args) {

    match options.type_to_save.as_str() {
        "disk" => {
            disk::save_disk();
        },
        "folder" => {
            folder::save_folder();
        },
        "files" => {
            files::save_files();
        },
        _ => {
            println!("Option unknow, options available : disk, folder, files")
        },
    }
}

fn choose_path_folder(r#type: &str) -> String {
    let mut path = String::from("/");
    loop {
        let mut vector_dirs: Vec<String> = Vec::new();
        
        vector_dirs.push(String::from("Valid"));
        vector_dirs.push(String::from("Back"));

        let dirs = fs::read_dir(path.clone()).unwrap();
                
        for dir in dirs {
            let dir = dir.unwrap();
            if dir.path().is_dir() {
                //TODO : sort ?
                let dir_path_string = dir.path().into_os_string().into_string().unwrap();
                let dir_splitted = dir_path_string.split("/").collect::<Vec<&str>>();

                //don't show hidden directories
                if !dir_splitted.iter().last().unwrap().starts_with("."){
                    vector_dirs.push(dir_path_string);
                }
            }
        }

        let directory: Result<String, InquireError> = Select::new(format!("The actual path is {path}, press Valid to confirm").as_str(), vector_dirs).prompt();

        match directory {
            Ok(choice) => {
                
                if choice == "Valid" {

                    if Confirm::new(format!("Are you sure you want to select {} as {type} folder ?", path).as_str()).with_default(false).prompt().unwrap() {
                        return path;
                    }

                } else if choice == "Back" {
                    let clone = path.clone();
                    let mut splits = clone.split("/").collect::<Vec<&str>>();

                    if splits.len() > 1 {
                        splits.pop();
                    }

                    path = String::from("/");
                    for split in splits.iter() {
                        if path == "/" {
                            path = path+format!("{}", split).as_str();
                        } else {
                            path = path+"/"+format!("{}", split).as_str();
                        }
                       
                    }
                   continue;

                } else {
                    path = choice;
                    continue;
                }
            }
            Err(e) => {
                println!("Error : {e}");
                std::process::exit(1);
            }
        }
    }
}

fn calculate_directory_size(path: &str) -> u64 {
    let mut folder_size = 0;

    let dirs = fs::read_dir(path).unwrap();

    for dir in dirs {
        let dir = dir.unwrap();
        let metadata = dir.metadata().unwrap();
        
        if metadata.is_dir() {
            let sub_dir_size = calculate_directory_size(dir.path().to_str().unwrap());
            folder_size += sub_dir_size;
        } else {
            folder_size += metadata.len();
        }
    }

    return folder_size; //return in bytes
}

fn get_available_space_disk(path: &str)-> u64 {
    let disk_dest = Command::new("df")
    .args([
        "--output=avail",
        path
        ]) .stdout(Stdio::piped())
    .output()
    .expect("failed to execute process");
    
    let output_str = String::from_utf8(disk_dest.stdout).unwrap();

    let mut available_space: Option<u64> = None;
    for line in output_str.lines() {
        if let Some(size_str) = line.trim().split_whitespace().next() {
            if let Ok(size) = size_str.parse::<u64>() {
                available_space = Some(size);
                break;
            }
        }
    }

    if let Some(size) = available_space {
        return size * 1024; //return in bytes
    } else {
        println!("Error when calculating the available space");
        std::process::exit(1);
    }
    
}