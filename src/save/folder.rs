use crate::save::choose_path_folder;

pub fn save_folder() {
    // TODO : ziper le dossier source
    println!("Choose the folder you want to save");
    let source = choose_path_folder("source");
    println!("Choose the destination");
    let destination = choose_path_folder("destination");

    println!("source: {source}, destination : {destination}");
}
