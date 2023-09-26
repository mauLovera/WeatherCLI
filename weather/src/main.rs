use clap::Parser;
use colored::Colorize;
use dotenv;
use serde::{Deserialize, Serialize};
#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();
    let api_key: String = dotenv::var("OPEN_WEATHER_API_KEY").unwrap();

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
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
        // humidity: f32,
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

    let response: CurrentWeather = reqwest::Client::new().get(url).send().await?.json().await?;
    let feels_like_emoji: &str;

    let is_hot_outside: bool = response.main.feels_like >= 80.0;
    let is_nice_outside: bool = response.main.feels_like < 80.0 && response.main.feels_like >= 60.0;

    if is_hot_outside {
        feels_like_emoji = "🥵🔥";
    } else if is_nice_outside {
        feels_like_emoji = "😎🔆";
    } else {
        feels_like_emoji = "🥶🧊";
    }

    let output: String = format!("{} The temperature in {} is {}°f, but it feels like {}°f {}",
        feels_like_emoji,
        response.name,
        response.main.temp as i32,
        response.main.feels_like as i32,
        feels_like_emoji
    );

    let colorized_output: colored::ColoredString = {
        if is_hot_outside {
            output.red()
        } else if is_nice_outside {
            output.yellow()
        } else {
            output.blue()
        }
    };

    println!("{}", colorized_output);

    Ok(())
}
