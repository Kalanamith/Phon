use exif_fixer::exif_data::exif_reader;
use std::env;

fn main() {
    // TODO: read version from .env file
    println!("Photo orientation fixer version 0.0.1");

    read_user_input();
}

/// Reads user input
/// Calls method `exif_fixer()`
fn read_user_input() ->Result<&'static str, ()> {
    let mut file_names: Vec<String> = env::args().collect();
    file_names.remove(0);
    if file_names.len() == 0 {
        // TODO: Panic here!
        return Ok("No file names were given")
    }

    match exif_reader(file_names) {
        Err(err) => println!("{:?}", err),
        _ => ()
    };
    Ok("Processing")
}

// TODO: Move this to another file
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_user_input_failure() {
        assert_eq!(read_user_input(), Ok(("No file names were given")));
    }
}