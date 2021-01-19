use actix_web::{
    post,
    web::{self},
    HttpResponse,
};
use serde_json::json;

use crate::models::server_error::{map_to_server_error, ServerError};

/// Post JSON to specified path
/// - This creates a json file in the project data dir(in the specified path) with the posted json data
#[post("/rest/{url_path:.*}")]
pub async fn post_json_to_path(
    url_path: web::Path<String>,
    req_body: String,
) -> Result<HttpResponse, ServerError> {
    let json_body: serde_json::Value =
        serde_json::from_str(&req_body).map_err(map_to_server_error)?;
    Ok(HttpResponse::Ok().json(json!({
       "path": url_path.to_string(),
       "body": json_body,
    })))
}
