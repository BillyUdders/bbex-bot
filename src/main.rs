use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

async fn hello_world(input_str: String) -> String {
    format!("Hello, {}", input_str)
}

async fn goodbye_world(input_str: String) -> String {
    format!("Goodbye, {}", input_str)
    "Hello, Josh! Get fucked!"
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    // Define the structure of the Weather object based on the JSON response from the API
    // This is just a placeholder, you should replace it with the actual structure
    // of the JSON response from the weather API.
    temperature: f64,
    humidity: f64,
    // Add more fields as needed
}

async fn get_weather() { 

    // Make a GET request to the weather API
    let response = reqwest::get("https://api.weatherapi.com/v1/current.json?key=8a6905d7a41046a486950857242102&q=London").await;
    let r = response.response; 
    // Check if the request was successful
    if r.status().is_success() {
        // Deserialize the JSON response into a Weather struct
        let weather: Weather = response.json().await?;
        
        Ok(weather)
    } else {
        // Print an error message if the request was not successful
        Err(format!("Failed to fetch weather data: {}", response.status()).into())
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
<<<<<<< HEAD
<<<<<<< HEAD
    Ok(Router::new()
        .route("/hello", get(hello_world))
        .route("/goodbye", get(goodbye_world))
        .into())
=======
    let router = Router::new().route("/", get(hello_world)).route("/fuck"., get(fuck_the_world));
    
=======
    let router = Router::new().route("/", get(hello_world)).route("/weather", get(get_weather));
>>>>>>> c21eeec (Locking)

    Ok(router.into())
>>>>>>> a6108bc (Adding go fuck yaself)
}

