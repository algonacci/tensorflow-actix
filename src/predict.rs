use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

#[derive(MultipartForm)]
pub struct Upload {
    file: TempFile,
}

pub async fn put_file(config: web::Data<Config>, form: MultipartForm<Upload>) -> impl Responder {
    const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10; // 10 MB
    const MAX_FILE_COUNT: i32 = 1;

    // reject malformed requests
    match form.file.size {
        0 => return HttpResponse::BadRequest().finish(),
        length if length > MAX_FILE_SIZE.try_into().unwrap() => {
            return HttpResponse::BadRequest().body(format!(
                "The uploaded file is too large. Maximum size is {} bytes.",
                MAX_FILE_SIZE
            ));
        }
        _ => {}
    };

    let temp_file_path = form.file.file.path();
    let file_name: &str = form
        .file
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    let mut file_path = PathBuf::from(&config.data_path);
    file_path.push(&sanitize_filename::sanitize(&file_name));

    match std::fs::rename(temp_file_path, file_path) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
