use std::{ffi::OsStr, path::Path};

/// Reads a BrainFuck file from a given path.
///
/// # Arguments
///
/// * `path` - The path to the `.bf` file.
///
/// # Returns
///
/// * `Result<String, std::io::Error>` - The contents of the file.
///
/// # Errors
///
/// * `InvalidInput` - If the file extension isn't `.bf`.
/// * `std::io::Error` - If any I/O error occurs while trying to read.
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
