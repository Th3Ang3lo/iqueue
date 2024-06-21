use std::fs::{self, OpenOptions};
use std::io::{Write, Error};

pub struct FileUtils;

pub trait FileUtilsImpl {
    fn file_exists(file_path: &str) -> Option<bool>;
    fn read_file_as_string(file_path: &str) -> Result<String, Error>;
    fn delete_file(file_path: &str) -> Option<bool>;
    fn append(file_path: &str, content: &str) -> Result<bool, Error>;
}

impl FileUtilsImpl for FileUtils {
    fn append(file_path: &str, content: &str) -> Result<bool, Error> {
        if let Some(parent_dir) = std::path::Path::new(&file_path).parent() {
            if !parent_dir.exists() {
                if let Err(err) = fs::create_dir_all(parent_dir) {
                    eprintln!("Erro ao criar o diretório: {}", err);

                    return Err(err);
                }
            }
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;

        if let Err(err) = writeln!(file, "{}", content) {
            eprintln!("Couldn't write to file: {}", err);
        }

        Ok(true)
    }

    fn delete_file(file_path: &str) -> Option<bool> {
        let file_exists = <FileUtils as FileUtilsImpl>::file_exists(file_path)?;

        if file_exists {
            let _ = fs::remove_file(file_path);

            return Some(true);
        }

        None
    }

    fn read_file_as_string(file_path: &str) -> Result<String, Error> {
        let file_content = fs::read_to_string(file_path)?;

        Ok(file_content)
    }

    fn file_exists(file_path: &str) -> Option<bool> {
        let metadata = fs::metadata(file_path);
        
        if metadata.unwrap().is_file() {
            return Some(true);
        }

        None
    }
}
