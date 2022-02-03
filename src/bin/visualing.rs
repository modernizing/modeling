use std::borrow::Cow;
use std::fs;

use actix_web::body::Body;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args_os().collect();

    let mut port = "9000";
    if args.len() > 1 {
        port = args[1].to_str().unwrap();
    }

    return start_local_server(port).await;
}

async fn start_local_server(port: &str) -> std::io::Result<()> {
    let url = format!("http://127.0.0.1:{}", port);
    println!("start server: {}", url);

    open_url(&url);
    return start(port).await;
}

pub async fn start(port: &str) -> std::io::Result<()> {
    return HttpServer::new(move || {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(data)
            .service(web::resource("/{_:.*}").route(web::get().to(dist)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await;
}

#[get("/output.json")]
pub fn data() -> HttpResponse {
    let content = fs::read_to_string("output.json").unwrap();

    return HttpResponse::Ok()
        .content_type("application/json")
        .body(content.into_bytes());
}

pub fn open_url(url: &str) {
    if let Err(err) = webbrowser::open(url) {
        println!("failure to open in browser: {}", err);
    }
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content.data {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

pub fn dist(path: web::Path<String>) -> HttpResponse {
    handle_embedded_file(&path.0)
}
