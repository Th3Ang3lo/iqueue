use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

use std::io::Error;

pub struct FileUtils;

impl FileUtils {
    pub fn append(file_path: &str, content: &str) -> Result<bool, Error> {
        if let Some(parent_dir) = std::path::Path::new(&file_path).parent() {
            if !parent_dir.exists() {
                if let Err(err) = fs::create_dir_all(parent_dir) {
                    eprintln!("Erro ao criar o diretório: {}", err);

                    return Err(err);
                }
            }
        }

        let file_result = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path);

        if let Err(err) = file_result {
            return Err(err);
        }

        let mut file = file_result.unwrap();

        if let Err(err) = writeln!(file, "{}", content) {
            eprintln!("Couldn't write to file: {}", err);
        }

        return Ok(true);
    }

    pub fn delete_file(file_path: &str) -> Result<(), Error> {
        return fs::remove_file(file_path);
    }

    pub fn read_file_as_string(file_path: &str) -> Result<String, Error> {
        let file = fs::read_to_string(file_path);

        if let Err(err) = file {
            panic!("Error opening file: {}", err);
        }

        return Ok(file.unwrap());
    }

    pub fn file_exists(file_path: &str) -> Option<bool> {
        if let Ok(metadata) = fs::metadata(file_path) {
            return Some(metadata.is_file());
        }

        return None;
    }
}
