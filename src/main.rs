use axum::{routing::get, Router};

async fn hello_world(input_str: String) -> String {
    format!("Hello, {}", input_str)
}

async fn goodbye_world(input_str: String) -> String {
    format!("Goodbye, {}", input_str)
    "Hello, Josh! Get fucked!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        .route("/hello", get(hello_world))
        .route("/goodbye", get(goodbye_world))
        .into())
}

