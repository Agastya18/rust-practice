use actix_web::{get,post,web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use nanoid::nanoid;
use std::env;
#[derive(Serialize, Deserialize)]
struct Url {
    original: String,
   
}

#[derive(Serialize, Deserialize)]
struct ShortenedUrl {
    shortened: String,
    original: String,}

struct AppState {
    urls: Mutex<HashMap<String, String>>,
}
#[get("/")]
async fn index()->impl Responder{
    HttpResponse::Ok().body("Welcome to the Actix server!")


}

#[post("/shorten")]
async fn shorten(data: web::Json<Url>, state: web::Data<AppState>) -> impl Responder {
    let original_url = data.original.clone();
    let id = nanoid!(4); // Generate a unique ID
    let shortened_url = format!("http://localhost:8080/{}", id);

    // Store the original URL with its shortened ID
    let mut urls = state.urls.lock().unwrap();
    urls.insert(id.clone(), original_url.clone());

    // Return both original and shortened URLs
    HttpResponse::Ok().json(ShortenedUrl {
        shortened: shortened_url,
        original: original_url,
    })
}

#[get("/{id}")]
async fn redirect(id: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let urls = state.urls.lock().unwrap();
    if let Some(original_url) = urls.get(&id.into_inner()) {
        HttpResponse::Found().append_header(("Location", original_url.as_str())).finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        urls: Mutex::new(HashMap::new()),
    });
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(shorten)
            .service(redirect)
            .service(index)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
