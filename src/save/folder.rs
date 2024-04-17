use crate::{utils, commands};
use inquire::{Confirm, ui::{Color, RenderConfig, StyleSheet}};

pub fn save_folder() {
    println!("Choose the folder you want to save");
    let source = utils::choose_path_folder("source");
    println!("Choose the destination");
    let destination = utils::choose_path_folder("destination");
    let mut delete_after = None;

    println!("Calculating directory size...");
    let folder_size = utils::calculate_directory_size(&source);
    
    let destination_available_size = utils::get_available_space_disk(destination.as_str());

    if folder_size > destination_available_size {
        println!("\x1b[31mError, the folder size is superior than the destination !\x1b[0m");
        std::process::exit(1);
    }
    let mut render_config = RenderConfig::default();
    render_config.prompt = StyleSheet::new().with_fg(Color::DarkYellow);

    if Confirm::new("Would you like to delete files on the destination that are no longer present on the source after synchronization?").with_render_config(render_config).with_default(false).prompt().unwrap() {
        delete_after = Some("--delete-after");
    }

    if Confirm::new(format!("Are you sure you want to save {} to {} ?", source, destination).as_str()).with_render_config(render_config).with_default(false).prompt().unwrap() {
        let mut args = vec!["-av", "--numeric-ids", "--progress", source.as_str(), destination.as_str()];
        if let Some(delete_option) = delete_after {
            args.push(delete_option);
        }

        match commands::rsync(args) {
            Ok(_) => {
                println!("\x1b[32mBackup completed successfully\x1b[0m");
            }
            Err(e) => {
                println!("\x1b[31mError : {}\x1b[0m", e.kind());
            }
        }
    } else {
        /*
            save_folder(); ?
        */
        println!("\x1b[33mAborted by user\x1b[0m");
        std::process::exit(1);
    }
}
