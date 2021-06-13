use std::{fmt};

const WEATHER_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

#[tokio::main]
async fn main() {
    let client = Client { weather_api_key: env!("WEATHER_API_KEY").to_string() };
    let args: Vec<String> = std::env::args().collect();

    match args.as_slice() {
        [_, city] => client.fetch_and_print_weather(&[city]).await,
        [_, city, region] => client.fetch_and_print_weather(&[city, region]).await,
        [_, city, region, country] => client.fetch_and_print_weather(&[city, region, country]).await,
        _ => print_help()
    };
}

fn print_help() {
    println!("Usage:");
    println!("  $ weather <city> [region] [country]");
}

struct Client {
    weather_api_key: String,
}

impl Client {
    async fn fetch_and_print_weather(&self, query: &[&str]) {
        let url = format!("{}?appid={}&q={}&units=imperial", WEATHER_API_BASE_URL, self.weather_api_key, query.join(","));
        let response = reqwest::get(url).await.unwrap();

        let status = response.status();
        if status.is_success() {
            println!("{}", response.json::<WeatherResponse>().await.unwrap());
        } else if status.is_client_error() {
            println!("an error occurred: {}", response.json::<WeatherError>().await.unwrap().message);
        } else {
            println!("an unknown error occurred")
        };
    }
}

#[derive(serde::Deserialize, fmt::Debug)]
struct WeatherResponse {
    name: String,
    main: Main,
    weather: Vec<Weather>,
    wind: Wind,
}

#[derive(serde::Deserialize, fmt::Debug)]
struct Weather {
    main: String,
    description: String,
}

#[derive(serde::Deserialize, fmt::Debug)]
struct Main {
    temp: f64,
    humidity: f64,
}

#[derive(serde::Deserialize, fmt::Debug)]
struct Wind {
    speed: f64,
}

impl fmt::Display for WeatherResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "weather in {}:\n  {}°F {}\n  humidity: {}%\n  wind speed: {}mph",
            self.name,
            self.main.temp,
            self.weather[0].description,
            self.main.humidity,
            self.wind.speed,
        )
    }
}

#[derive(serde::Deserialize, fmt::Debug)]
struct WeatherError {
    message: String,
}
