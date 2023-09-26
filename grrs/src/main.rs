use dotenv;
use serde::{Deserialize, Serialize};
#[tokio::main]


async fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();
    let api_key: String = dotenv::var("OPEN_WEATHER_API_KEY").unwrap();
    
    const LAT: f32 = 38.8895;
    const LON: f32 = -77.0353;
    
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

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={LAT}&lon={LON}&appid={api_key}&units=imperial"
    );

    let response: CurrentWeather = reqwest::Client::new().get(url).send().await?.json().await?;

    println!("{:#?}", response);

    Ok(())
}
