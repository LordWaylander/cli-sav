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
    println!("restore something : {:?}", options)
}