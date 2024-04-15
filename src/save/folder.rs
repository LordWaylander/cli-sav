use crate::save::{choose_path_folder, calculate_directory_size, get_available_space_disk};
use std::process::Command;
use inquire::Confirm;

pub fn save_folder() {
    println!("Choose the folder you want to save");
    let source = choose_path_folder("source");
    println!("Choose the destination");
    let destination = choose_path_folder("destination");
    let mut delete_after = None;

    println!("Calculating directory size...");
    let folder_size = calculate_directory_size(&source);
    
    let destination_available_size = get_available_space_disk(destination.as_str());

    if let Err(e) = destination_available_size {
        println!("Error {e}");
        std::process::exit(1);
    }

    if Ok(folder_size) > destination_available_size {
        println!("Error, the folder size is superior than the destination !");
        std::process::exit(1);
    }


    if Confirm::new("Would you like to delete files on the destination that are no longer present on the source after synchronization?").with_default(false).prompt().unwrap() {
        delete_after = Some("--delete-after");
    }

    if Confirm::new(format!("Are you sure you want to save {} to {} ?", source, destination).as_str()).with_default(false).prompt().unwrap() {
        let mut args = vec!["-av", "--numeric-ids", "--progress", source.as_str(), destination.as_str()];
        if let Some(delete_option) = delete_after {
            args.push(delete_option);
        }

        match Command::new("rsync").args(args).status() {
            Ok(_) => {
                println!("Backup completed successfully");
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
