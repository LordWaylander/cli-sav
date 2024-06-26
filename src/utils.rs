use inquire::{Select, error::InquireError, Confirm, ui::{Color, RenderConfig, StyleSheet}};
use std::fs::{self, File};
use std::process::{Command, Stdio};
use std::path::Path;
use std::ffi::OsStr;

pub fn choose_image_disk() -> String {
    let mut path = String::from("/");
    loop {
        let mut vector_dirs: Vec<String> = Vec::new();
        vector_dirs.push(String::from("Back"));

        match fs::read_dir(path.clone()) {
            Ok(dirs) => {
                for dir in dirs {
                    let dir = dir.unwrap();
                    let dir_path_string = dir.path().into_os_string().into_string().unwrap();
                    let dir_splitted = dir_path_string.split("/").collect::<Vec<&str>>();

                    if dir.path().is_dir() {
                        //TODO : sort ?
                        //don't show hidden directories
                        if !dir_splitted.iter().last().unwrap().starts_with("."){
                            vector_dirs.push(dir_path_string);
                        }
                    } else {
                        let extention = Path::new(dir_path_string.as_str()).extension().and_then(OsStr::to_str);

                        if let Some(extention)= extention {
                            if extention == "img" {
                                vector_dirs.push(dir_path_string);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("\x1b[31mError : {}, choose another folder\x1b[0m", e.kind());
                path = String::from("/");
                continue;
            }
        }

        let directory: Result<String, InquireError> = Select::new(format!("The actual path is {path}, select image disk to restore").as_str(), vector_dirs).prompt();

        match directory {
            Ok(choice) => {
                
                if choice == "Back" {
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
                    match File::open(choice.clone()) {
                        Ok(file)=> {
                            
                            
                            if file.metadata().unwrap().file_type().is_file() {
                                let mut render_config = RenderConfig::default();
                                render_config.prompt = StyleSheet::new().with_fg(Color::DarkYellow);

                                if Confirm::new(format!("Are you sure you want to select {} as image disk ?", choice).as_str()).with_render_config(render_config).with_default(false).prompt().unwrap() {
                                    return choice;
                                } else {
                                    continue;
                                }
                            }else {
                                path = choice;
                            }
                        }
                        Err(e) => {
                            println!("\x1b[31mError {}\x1b[0m", e.kind());
                            std::process::exit(1);
                        }
                    }
                    continue;
                }
            }
            Err(e) => {
                println!("\x1b[31mError : {e}\x1b[0m");
                std::process::exit(1);
            }
        }
    }
}

pub fn choose_path_folder(r#type: &str) -> String {
    let mut path = String::from("/");
    loop {
        let mut vector_dirs: Vec<String> = Vec::new();
        
        vector_dirs.push(String::from("Valid"));
        vector_dirs.push(String::from("Back"));

        match fs::read_dir(path.clone()) {
            Ok(dirs) => {
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
            }
            Err(e) => {
                println!("\x1b[31mError : {}, choose another folder\x1b[0m", e.kind());
                path = String::from("/");
                continue;
            }
        }

        let directory: Result<String, InquireError> = Select::new(format!("The actual path is {path}, press Valid to confirm").as_str(), vector_dirs).prompt();

        match directory {
            Ok(choice) => {
                
                if choice == "Valid" {
                    let mut render_config = RenderConfig::default();
                    render_config.prompt = StyleSheet::new().with_fg(Color::DarkYellow);

                    if Confirm::new(format!("Are you sure you want to select {} as {type} folder ?", path).as_str()).with_render_config(render_config).with_default(false).prompt().unwrap() {
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
                println!("\x1b[31mError : {e}\x1b[0m");
                std::process::exit(1);
            }
        }
    }
}

pub fn calculate_directory_size(path: &str) -> u64 {
    let mut folder_size = 0;

    match fs::read_dir(path) {
        Ok(dirs) => {
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
        }
        Err(e) => {
            println!("\x1b[31mCan't calculate size of {} reason : {}\x1b[0m", path, e.kind());
        }
    }
    return folder_size; //return in bytes
}

pub fn calculate_file_size(path: &str) -> u64 {
    match File::open(path) {
        Ok(file)=> {
            return file.metadata().unwrap().len();
        }
        Err(e) => {
            println!("\x1b[31mCan't calculate size of {} reason : {}\x1b[0m", path, e.kind());
            std::process::exit(1);
        }
    }
}

pub fn get_available_space_disk(path: &str)-> u64 {
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
        println!("\x1b[31mError when calculating the available space\x1b[0m");
        std::process::exit(1);
    }
    
}