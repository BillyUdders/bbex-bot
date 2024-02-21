use axum::{routing::get, Router};

async fn hello_world(input_str: String) -> String {
    format!("Hello, {}", input_str)
}

async fn goodbye_world(input_str: String) -> String {
    format!("Goodbye, {}", input_str)
    "Hello, Josh! Get fucked!"
}

async fn fuck_the_world(arg: Type) -> &'static str {
    "Fuck the world"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
<<<<<<< HEAD
    Ok(Router::new()
        .route("/hello", get(hello_world))
        .route("/goodbye", get(goodbye_world))
        .into())
=======
    let router = Router::new().route("/", get(hello_world)).route("/fuck"., get(fuck_the_world));
    

    Ok(router.into())
>>>>>>> a6108bc (Adding go fuck yaself)
}

