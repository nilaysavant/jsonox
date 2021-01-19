use std::path::PathBuf;

use crate::{
    constants::paths::APP_DATA_DIR,
    models::server_error::{map_to_server_error, ServerError},
    utils::fsutils::{read_from_path, remove_from_path, write_to_path},
};
use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse,
};
use relative_path::RelativePath;
use serde_json::json;
use walkdir::WalkDir;

/// List all active paths
/// - List all available json paths
#[get("/")]
pub async fn list_active_paths() -> Result<HttpResponse, ServerError> {
    let file_pathbufs = WalkDir::new(APP_DATA_DIR)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().is_ok())
        .filter(|e| e.metadata().unwrap().is_file())
        .map(|e| e.path().to_owned())
        .collect::<Vec<PathBuf>>();

    let active_paths = file_pathbufs
        .into_iter()
        .filter(|e| e.parent().is_some())
        .filter_map(|e| {
            let path = e.parent().unwrap().strip_prefix(APP_DATA_DIR);
            match path {
                Ok(_) => match path.unwrap().to_str() {
                    Some(p) => Some(p.to_string()),
                    None => None,
                },
                Err(_) => None,
            }
        })
        .collect::<Vec<String>>();

    Ok(HttpResponse::Ok().json(json!({ "active_paths": active_paths })))
}

/// Post JSON to specified path
/// - This creates a json file in the project data dir(in the specified path) with the posted json data
#[post("/{url_path:.*}")]
pub async fn post_json_to_path(
    url_path: web::Path<String>,
    req_body: String,
) -> Result<HttpResponse, ServerError> {
    if url_path.len() > 0 {
        let file_path = RelativePath::new(url_path.as_str())
            .join("index.json")
            .normalize()
            .to_path(APP_DATA_DIR)
            .to_owned();
        if file_path.starts_with(APP_DATA_DIR) {
            let body: serde_json::Value =
                serde_json::from_str(&req_body).map_err(map_to_server_error)?;
            write_to_path(file_path.as_path(), &body)?;
            Ok(HttpResponse::Ok().json(body))
        } else {
            Err(ServerError::UserError {
                message: "JSON file path invalid!".to_string(),
            })
        }
    } else {
        Err(ServerError::UserError {
            message: "url path invalid!".to_string(),
        })
    }
}

/// Get JSON from specified path
/// - This reads a json file in the project data dir(in the specified path) retrieves the initially posted json data
#[get("/{url_path:.*}")]
pub async fn get_json_from_path(url_path: web::Path<String>) -> Result<HttpResponse, ServerError> {
    if url_path.len() > 0 {
        let file_path = RelativePath::new(url_path.as_str())
            .join("index.json")
            .normalize()
            .to_path(APP_DATA_DIR)
            .to_owned();
        if file_path.starts_with(APP_DATA_DIR) {
            let json_string = read_from_path(file_path.as_path())?;
            let json_data: serde_json::Value =
                serde_json::from_str(&json_string).map_err(map_to_server_error)?;
            Ok(HttpResponse::Ok().json(json_data))
        } else {
            Err(ServerError::UserError {
                message: "JSON file path invalid!".to_string(),
            })
        }
    } else {
        Err(ServerError::UserError {
            message: "url path invalid!".to_string(),
        })
    }
}

/// Delete JSON from specified path
/// - This reads a json file in the project data dir(in the specified path) retrieves the initially posted json data
#[delete("/{url_path:.*}")]
pub async fn delete_json_from_path(
    url_path: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    if url_path.len() > 0 {
        let file_path = RelativePath::new(url_path.as_str())
            .join("index.json")
            .normalize()
            .to_path(APP_DATA_DIR)
            .to_owned();
        if file_path.starts_with(APP_DATA_DIR) {
            let json_string = remove_from_path(file_path.as_path())?;
            let json_data: serde_json::Value =
                serde_json::from_str(&json_string).map_err(map_to_server_error)?;
            Ok(HttpResponse::Ok().json(json_data))
        } else {
            Err(ServerError::UserError {
                message: "JSON file path invalid!".to_string(),
            })
        }
    } else {
        Err(ServerError::UserError {
            message: "url path invalid!".to_string(),
        })
    }
}
