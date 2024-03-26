use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, Result};
use futures::{StreamExt, TryStreamExt};

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let upload_dir = "./uploads";
    std::fs::create_dir_all(upload_dir).ok();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let file_path = format!("{}/uploaded_file.jpg", upload_dir);
        let mut file_bytes = Vec::new();

        // Accumulate the chunks into a Vec<u8>
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file_bytes.extend_from_slice(&data);
        }

        // Write the accumulated bytes to the file
        let _ = web::block(move || std::fs::write(file_path, file_bytes)).await?;
    }

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("File uploaded successfully"))
}
