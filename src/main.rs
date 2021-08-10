use structopt::StructOpt;
use exitfailure::{ExitFailure};
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env; //-> Doesn't work
use dotenv::dotenv;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
    //cli_help: String -> Implement feature for later
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}
#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let place =  (&args.city, &args.country_code);
    let resp = Forecast::get(place).await?;
    let temp_cel = kelvin_to_celcius(resp.main.temp);
    let wind_speed = miles_per_sec_to_kmh(resp.wind.speed);
    let wind_direction = degrees_to_compass(resp.wind.deg);
    println!("🌍 {} => 🌡 Temp: {:.2}:, 💦 Humidity: {}%, 🌧 Clouds: {}, 🍃 Wind Speed: {:.2}, 🧭 Wind Direction: {}", place.0, temp_cel, resp.main.humidity, resp.weather.details.description, wind_speed, wind_direction);
    Ok(())
}

fn kelvin_to_celcius(kel: f64) -> f64{
    kel - 273.15
}
fn degrees_to_compass(deg: i32) -> & 'static str {
    return match deg {
        00..=21 => "North",
        22..=43 => "North Northeast",
        44..=45 => "North East",
        46..=66 => "East Northeast",
        67..=111 => "East",
        112..=133 => "East Southeast",
        134..=135 => "Southeast",
        136..=156 => "South Southeast",
        157..=201 => "South",
        202..=223 => "South Southwest",
        224..=225 => "Southwest",
        226..=246 => "West Southwest",
        247..=291 => "West",
        292..=313 => "West Northwest",
        314..=315 => "Northwest",
        316..=336 => "North Northewest",
        337..=360 => "North",
        _ => "Error getting wind direction",
    }
}
fn miles_per_sec_to_kmh(mph_to_kmh: f64) -> f64 {
    mph_to_kmh * 3.6
}

impl Forecast {
    async fn get(place: (&String,&String)) -> Result<Self,ExitFailure>{
        let api_key: &str = ""; //Change it to .env later

        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}", place.0, place.1, &value/*env::var("API_KEY").unwrap()*/);

        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(resp)
    }
}