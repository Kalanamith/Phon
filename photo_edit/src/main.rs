use exif_fixer::exif_data::exif_reader;
use std::env;

fn main() {
    // TODO: read version from .env file
    println!("Photo orientation fixer version 0.0.1");

    // TODO: move this to separate function
    // Capture panic here and log a message
    read_user_input()
}

/// Reads user input
/// Calls method `exif_fixer()`
fn read_user_input() {
    let mut file_names: Vec<String> = env::args().collect();

    if file_names.len() == 0 {
        // TODO: Panic here!
        println!("No file names were given");
    } else {
        file_names.remove(0);
    }


    exif_reader(file_names);

}
