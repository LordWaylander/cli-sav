use clap::Parser;
use std::process::Command;
/// Liste l'arboresence du dique
/// option 1 : niveau de profondeur
/// option 2 : point de départ de l'arboresence

#[derive(Parser)]
#[derive(Debug)]
pub struct Args {
    #[arg(index = 1, required = false, default_value = "1")]
    pub level_deep: String,
    #[arg(index = 2, required = false, default_value = "/")]
    pub folder: String,
}


pub fn main(options: Args) {
    Command::new("tree")
    .args([
        "-dxqpL",
        options.level_deep.as_str(), 
        options.folder.as_str()
    ])
    .status()
    .expect("failed to execute process");
}