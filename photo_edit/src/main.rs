use exif_fixer::exif_data::exif_reader;
use std::env;

fn main() {
    // TODO: read version from .env file
    println!("Photo orientation fixer version 0.0.1");

    // TODO: move this to separate function
    // Capture panic here and log a message
    // match read_user_input() {
    //     Err(err) => println!("Error processing images {}", err),
    //     _ => println!("Processing")
    // }
    read_user_input()
}

/// Reads user input
/// Calls method `exif_fixer()`
fn read_user_input() {
    let mut file_names: Vec<String> = env::args().collect();
    file_names.remove(0);
    if file_names.len() == 0 {
        // TODO: Panic here!
        println!("No file names were given");
    }
    // else {
    //
    // }

    match exif_reader(file_names) {
        Err(err) => println!("{:?}", err),
        _ => ()
    }
}
