use crate::error::ImagextractorError::{IOError, InvalidArgs};
use crate::extractor::ImageInfo;
use std::error::Error;
use std::path::Path;
use std::{env, fs};
use std::process;

mod error;
mod extractor;

fn main() {
    match env::args().collect::<Vec<String>>().split_first() {
        Some((_, args)) => {
            if args.len() <= 0 {
                eprintln!("Invalid arguments");
            } else {
                for arg in args {
                    validate(arg).unwrap_or_else(|e| {
                        eprintln!("Processing input <{}> error - {}", arg, e);
                        process::exit(1)
                    });
                    extractor::extract_img_info(arg);
                }
            }
        }
        None => eprintln!("Invalid arguments"),
    }
}

fn validate(file_name: &str) -> Result<(), error::ImagextractorError> {
    let path = Path::new(file_name);
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Err(InvalidArgs(
                    "The input arguments is a directory rather than an image file.".to_string(),
                ));
            }

            if !file_name.to_lowercase().ends_with(".jpg")
                && !file_name.to_lowercase().ends_with(".jpeg")
            {
                return Err(InvalidArgs(
                    "The input arguments must be a valid .jpg or .jpeg image.".to_string(),
                ));
            }
            Ok(())
        }
        Err(e) => Err(IOError(e)),
    }
}
