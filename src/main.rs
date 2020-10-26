#![feature(test)]
use crate::error::{
    ImagextractorError, ImagextractorError::IOError, ImagextractorError::InvalidArgs,
};
use std::path::Path;
use std::{env, fs, process};

mod benches;
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
                        eprintln!("Processing input [{}] error - {}", arg, e);
                        process::exit(1)
                    });
                    extractor::process(arg).unwrap_or_else(|e| {
                        eprintln!("Processing input [{}] error - {}", arg, e);
                        process::exit(1)
                    });
                }
            }
        }
        None => eprintln!("Invalid arguments"),
    }
}

fn validate(file_name: &str) -> Result<(), ImagextractorError> {
    let path = Path::new(file_name);
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Err(InvalidArgs(format!(
                    "The input argument {} is a directory rather than an image file.",
                    file_name
                )));
            }

            if !file_name.to_lowercase().ends_with(".jpg")
                && !file_name.to_lowercase().ends_with(".jpeg")
            {
                return Err(InvalidArgs(format!(
                    "The input argument {} must be a valid .jpg or .jpeg image.",
                    file_name
                )));
            }
            Ok(())
        }
        Err(e) => return Err(IOError(e)),
    }
}
