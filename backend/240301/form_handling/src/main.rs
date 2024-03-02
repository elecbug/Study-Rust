use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use actix_web::web::{Either, Json, Form};

// <easy-form-handling>
#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

// register form is JSON
async fn json_register(form: web::Json<Register>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(include_str!("../static/response.html"), form.username, form.country))
}

// register form can be either JSON or URL-encoded
async fn register(form: Either<Json<Register>, Form<Register>>) -> HttpResponse {
    let Register { username, country } = form.into_inner();
    
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(include_str!("../static/response.html"), username, country))
}
// </easy-form-handling>

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/register", web::post().to(register))
            .route("/json_register", web::post().to(json_register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}