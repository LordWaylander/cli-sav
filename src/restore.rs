use clap::Parser;
//use std::process::Command;

/// restore a Disk, folder or files
/// option required : "disk", "folder" or "files"

#[derive(Parser)]
#[derive(Debug)]
pub struct Args {
    #[arg(index = 1, required = true)]
    /// disk, folder, files
    pub type_to_restore: String,
}

pub fn main(options: Args) {
    // rzstore disk -> search le .img dans le select folder (afficher uniquement les .img)
    // rstore folder... same que sauvegarde
    
    println!("restore something : {:?}", options)
}