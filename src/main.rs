use actix_web::{web, put, get, App, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use futures_util::StreamExt;

// src/utils.rs
mod utils;
use utils::*;

/*
    This method takes a PUT request and uploads the file to the server
    The file is saved in the /tmp directory and given a UUID name
*/
#[put("/upload")]
async fn upload(mut payload: web::Payload) -> impl Responder {
    
    // Probably stupid... but I have no idea how to get the payload size in bytes
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        let data = item.unwrap();
        bytes.extend_from_slice(&data);
    }

    // If the payload is empty (i.e. no file was sent) return an error
    if bytes.is_empty() {
        warning("Empty / No file was sent");
        return HttpResponse::BadRequest().body("File is empty\n");
    }

    // Else save the file to the /tmp directory
    let file_name = generate_uuid();
    let mut file = File::create(format!("/tmp/{}", file_name)).unwrap();
    while let Some(chunk) = payload.next().await {
        let data = chunk.unwrap();
        file.write_all(&data).unwrap();
    }

    success(format!("File saved as {}", file_name).as_str());
    HttpResponse::Ok().body(format!("File uploaded as {}\n", file_name))
}

// If no file is specified, return a list of files
#[get("/download")]
async fn list() -> impl Responder {
    let mut files = String::new();
    for entry in std::fs::read_dir("/tmp").unwrap() {
        let entry = entry.unwrap();
        files.push_str(&entry.file_name().to_str().unwrap());
        files.push_str("\n");
    }

    success("Sent file listing");
    HttpResponse::Ok().body(files)
}

/* 
    This method takes a GET request and returns the contents of the file
    specified in the URL
*/
#[get("/download/{file_name}")]
async fn download(file_name: web::Path<String>) -> impl Responder {
    let path = format!("/tmp/{}", file_name);

    if File::open(&path).is_ok() {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        success(format!("Sent file {}", file_name).as_str());
        return HttpResponse::Ok().body(contents);
    } else {
        error(format!("File {} does not exist", file_name).as_str());
        return HttpResponse::NotFound().body("File not found\n");
    }
}

// Start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Start the server
    success("Starting server");
    HttpServer::new(|| {
        App::new()
            .service(upload)
            .service(list)
            .service(download)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
