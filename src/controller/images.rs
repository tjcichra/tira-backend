use super::TiraError;
use crate::service;
use anyhow::Result;
use axum::{
    extract::{Multipart, Path},
    response::{IntoResponse, Response},
};

// Allow bigger than 2MB: https://docs.rs/axum/latest/axum/extract/struct.Multipart.html#large-files
pub async fn upload_image_endpoint(
    Path(file_name): Path<String>,
    mut multipart: Multipart,
) -> Result<Response, TiraError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        // let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        service::images::upload_image(&file_name, data.to_vec()).await;
    }
    Ok("ok".into_response())
}

pub async fn retrieve_image_endpoint(Path(file_name): Path<String>) -> Result<Response, TiraError> {
    let data = service::images::load_image(&file_name).await;
    Ok(data.into_response())
}
