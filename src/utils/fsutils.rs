use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use crate::{
    constants::paths::APP_DATA_DIR,
    models::server_error::{map_to_server_error, ServerError},
};

/// Read json file from a path and return contents as string
pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<String, ServerError> {
    let json_string = fs::read_to_string(path).map_err(map_to_server_error)?;
    Ok(json_string)
}

/// Write `json_data` to the specified file path (recursively create all parent paths)
pub fn write_to_path<P: AsRef<Path> + Copy>(
    path: P,
    json_data: &serde_json::Value,
) -> Result<(), ServerError> {
    let parent_path = path.as_ref().parent().unwrap_or(Path::new(APP_DATA_DIR));
    fs::create_dir_all(parent_path).map_err(map_to_server_error)?;
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())
        .map_err(map_to_server_error)?;
    f.write_all(
        serde_json::to_string_pretty(json_data)
            .map_err(map_to_server_error)?
            .as_ref(),
    )
    .map_err(map_to_server_error)?;
    f.sync_all().map_err(map_to_server_error)?;
    Ok(())
}

/// Remove parent dir in which json file is preset from a path and return the json contents as string
pub fn remove_from_path<P: AsRef<Path> + Copy>(path: P) -> Result<String, ServerError> {
    let json_string = fs::read_to_string(path).map_err(map_to_server_error)?;
    let parent_path = path.as_ref().parent().unwrap_or(Path::new(APP_DATA_DIR));
    fs::remove_dir_all(parent_path).map_err(map_to_server_error)?;
    Ok(json_string)
}
