use std::env;

fn main() {
    // TODO: read version from .env file
    println!("Photo orientation fixer version 0.0.1");

    // TODO: move this to separate function
    let mut file_name_args: Vec<String> = env::args().collect();
    file_name_args.remove(0);
    if file_name_args.len() == 0 {
        println!("No file names were given");
    } else {
        file_name_args.remove(0);
    }

}
