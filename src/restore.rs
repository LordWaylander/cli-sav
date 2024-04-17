use clap::Parser;
mod disk;
mod folder;

/// restore a Disk, Folder
/// option required : "disk" (sudo rights necessary), "folder"

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(index = 1, required = true)]
    /// disk, folder, files
    pub type_to_restore: String,
}

pub fn main(options: Args) {

    match options.type_to_restore.as_str() {
        "disk" => {
            disk::restore_disk();
        },
        "folder" => {
            folder::restore_folder();
        },
        /*"files" => {
            files::restore_files();
        },*/
        _ => {
            println!("Option unknow, options available : disk, folder, files")
        },
    }
}