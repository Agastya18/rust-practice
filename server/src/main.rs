use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result };
#[warn(unused_imports)]
#[get("/")]
async fn index()->impl Responder{
    HttpResponse::Ok().body("Welcome to the Actix server!")


}
#[get("/hell")]
async fn hell()->impl Responder{
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]

async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(index)
            .service(hell)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}