use std::io;
use reqwest::blocking::get;
use serde::Deserialize; 
use colored::*;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    name: String,
}
#[derive(Deserialize)]
struct Main {
    temp: f32,
   
    humidity: u32,
}


#[derive(Deserialize)]
struct Weather {
    description: String,
}

fn main() {
   
  println!(" welcome to the weather application");
  let mut city = String::new();

  println!("enter the city name:");
  io::stdin().read_line(&mut city).expect("Failed to read line"); 

  let api = "67c83b1e5fd973ba5633e44d42558ce3";

  let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},&appid={}&units=metric", city.trim(),api);

  let response:WeatherResponse =get(&url).expect("Failed to get response").json().expect("Failed to parse json");

  // Log the response
 // println!("{:?}", response);
   
  println!(
    "{} in {}: {}°C, {}",
    response.weather[0].description.green(), // Colorize weather description in green
    response.name.blue(), // Colorize city name in blue
    response.main.temp, // Display temperature in Celsius
    response.main.humidity // Display humidity percentage
);
}

