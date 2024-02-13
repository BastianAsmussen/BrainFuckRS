use std::{ffi::OsStr, path::Path};

pub fn read_bf_file(path: &str) -> Result<String, std::io::Error> {
    let path = Path::new(path);
    if path.extension() == None || path.extension() != Some(&OsStr::new("bf")) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File must have a .bf extension!",
        ));
    }

    std::fs::read_to_string(path)
}
