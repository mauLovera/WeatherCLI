use clap::Parser;
use dotenv;
use serde::{Deserialize, Serialize};
#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();
    let api_key: String = dotenv::var("OPEN_WEATHER_API_KEY").unwrap();

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// Name of the person to greet
        #[arg(long, default_value = "Mauricio")]
        name: String,

        /// Name of the city to lookup
        #[arg(short, long, default_value = "Miami")]
        city: String,

    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Coord {
        lat: f32,
        lon: f32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Weather {
        id: u32,
        main: String,
        description: String,
        icon: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Main {
        temp: f32,
        feels_like: f32,
        humidity: f32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct CurrentWeather {
        coord: Coord,
        weather: Vec<Weather>,
        main: Main,
        name: String,
    }

    let args: Args = Args::parse();
    let city_name: String = args.city;
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city_name}&appid={api_key}&units=imperial"
    );

    let response: CurrentWeather = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    println!(
        "Hello {}! The temperature in {} is {}°f, but it feels like {}°f",
        args.name, response.name, response.main.temp as i32, response.main.feels_like as i32
    );

    Ok(())
}
