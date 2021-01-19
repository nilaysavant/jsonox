use crate::{
    constants::paths::APP_DATA_DIR,
    models::server_error::{map_to_server_error, ServerError},
};
use actix_web::{
    post,
    web::{self},
    HttpResponse,
};
use relative_path::RelativePath;

/// Post JSON to specified path
/// - This creates a json file in the project data dir(in the specified path) with the posted json data
#[post("/rest/{url_path:.*}")]
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
