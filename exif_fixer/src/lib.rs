/// Module declaration to read exif data from a given image
/// and fix orientation.
pub mod exif_data {
    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;
    use exif::{Exif, In, Reader, Tag};

    pub fn exif_reader(file_names: Vec<String>) -> std::io::Result<()>{
        for fname in file_names {
            if Path::new(&fname).is_file(){
                let file = File::open(&fname).unwrap();

                let reader = Reader::new();
                let mut buf_reader = BufReader::new(&file);

                let exif_data =  reader.read_from_container(&mut buf_reader);
                let _exif_data = match exif_data {
                    Ok(exif) => {
                        get_orientation(&fname, exif)
                    },
                    Err(_err) => {
                        println!("Exif data  not found");
                        return Ok(())
                    }
                };
            }
            else {
                println!("Path not found");
            }
        }

        Ok(())
    }

    /// Passes exif data
    fn get_orientation(fname: &String, exif: Exif) {
        if let Some(field) = exif.get_field(Tag::Orientation, In::PRIMARY) {
            if let Some(orientation) = field.value.get_uint(0) {
                println!("orientation {}.", orientation);

                match_orientation(&fname, orientation)
            }
        }
    }

    /// matches orientation and pass it to `exif_fixer()` which fix orientation
    fn match_orientation(fname: &&String, orientation: u32) {
        match orientation {
            8 | 3 | 6 => {
                println!("Correction needed");
                exif_fixer(&fname, orientation);
            },
            _ => println!("Correction not needed"),
        }
    }

    /// Fix orientation of a file and writes back without exif data
    fn exif_fixer(file_path: &str, orientation: u32) {
        println!("Orientation is {}", orientation);

        let img = image::open(file_path).unwrap();

        let new_file_path = file_path;

        if orientation == 3 {
            image::imageops::rotate180(&img).save(&new_file_path).unwrap();
        }
        else if orientation == 6 {
            image::imageops::rotate90(&img).save(&new_file_path).unwrap();
        }
        else if orientation == 8 {
            image::imageops::rotate270(&img).save(&new_file_path).unwrap();
        }
    }
}

