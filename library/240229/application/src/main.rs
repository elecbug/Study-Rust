// use actix_web::{web, App, HttpServer, Responder};

// async fn index() -> impl Responder {
//     "Hello world!"
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             // prefixes all resources and routes attached to it...
//             web::scope("/app")
//                 // ...so this handles requests for `GET /app/index.html`
//                 .route("/index.html", web::get().to(index)),
//         )
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use actix_web::{get, web, App, HttpServer};

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}