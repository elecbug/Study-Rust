//! Run with
//!
//! ```not_rust
//! cargo run -p example-form
//! ```

use axum::{extract::Form, response::Html, routing::{get, post}, Router};
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_form=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = Router::new().route("/", get(show_form).post(accept_form));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(include_str!("./form.html"))
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    username: String,
    country: String,
}

async fn accept_form(Form(input): Form<Input>) -> Html<String> {
    Html(format!(include_str!("./form2.html"), input.country, input.username))
}
