use clap::Parser;

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