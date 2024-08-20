use colored::*;
use reqwest::Response;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]

struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]

struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]

struct Wind {
    speed: f64,
}

//function to get the weather information from OpenWeatherMap API
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=matric&appid={}",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = Response.json::<WeatherResponse>()?;
    Ok(response_json);
}

// function to display the weather information

fn display_weather_info(response: &WeatherResponse) {
    //extract the weather information from the response
    let description: &String = &response.weather[0].description;
}
