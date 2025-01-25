use std::io::{self, Write};
use std::fs;
use serde::{Serialize, Deserialize};
use chrono::Local;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct RunEntry {
    date: String,
    duration_formatted: String,
    exertion_rate: u8,
    temperature_celcius: f32,
    comments: String,
    pace: String,
    distance: f32,
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()

}

fn get_float_input(prompt: &str) -> f32 {
     loop {
         let input = get_user_input(prompt);
         match input.parse::<f32>() {
         Ok(value) => return value,
         Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn get_exertion_rate() -> u8 {
    loop {
       let input = get_user_input("Enter rate of exertion (1-10): ");
       match input.parse::<u8>() {
          Ok(value) if value >= 1 && value <=10 => return value, _=> println!("Please enter a number between 1 and 10."), 
       }
    }
}

fn main() {

    let distance = get_float_input("Enter in total distance: ");
    let duration = get_user_input("Enter run duration as HH:MM:SS or MM:SS: ");
    let exertion = get_exertion_rate();
    let temperature = get_float_input("Enter temperature (Celcius) ");
    let comments = get_user_input("Enter any comments: ");
    
    let pace_result = match calculate_pace(&duration, distance) {
        Ok((minutes, seconds)) => format_pace(minutes, seconds),
        Err(e) => {
           println!("Error calculating pace: {}", e);
           return;
        }// Get the pace tuple
     
    };

    let entry = RunEntry {
        date: Local::now().format("%Y-%m-%d").to_string(),
        duration_formatted: duration,  // Use correct field name
        exertion_rate: exertion,
        temperature_celcius: temperature,
        comments,
        pace: pace_result,            // Use the formatted string
        distance,
    };

    let base_path = Path::new("/Users/jfk/Documents/runjournal/2025");
    let filename = base_path.join(format!("run_{}.json", Local::now().format("%Y-%m-%d")));

    match serde_json::to_string_pretty(&entry) {

    Ok(json) => {
       if let Err(e) = fs::write(&filename, json) {
           println!("Error writing to file: {}", e);
       } else {
           println!("Run logged successfully to {}", filename.display());
       }
    }
    Err(e) => println!("Error creating JSON: {}", e),
    }
}
fn parse_time_to_seconds(time_str: &str) -> Result<f32, &'static str> {
    let parts: Vec<&str> = time_str.split(':').collect();
    
     match parts.len() {

     2 => {
         let minutes: f32 = parts[0].parse().map_err(|_| "Invalid minutes")?;
         let seconds: f32 = parts[1].parse().map_err(|_| "Invalid Seconds")?;
         
         Ok(minutes * 60.0 + seconds)
     },
     
     3 => {
         let hours: f32 = parts[0].parse().map_err(|_| "Invalid hours")?;
         let minutes: f32 = parts[1].parse().map_err(|_| "Invalid minutes")?;
         let seconds: f32 = parts[2].parse().map_err(|_| "Invalid seconds")?;

         Ok(hours * 3600.0 + minutes * 60.0 + seconds)
     },
    _ => Err("Invalid time format. Expected either MM:SS or HH:MM:SS")
    }
}

fn calculate_pace(time_str: &str, distance_km: f32) -> Result<(u32, u32), &'static str> {
    let total_seconds = parse_time_to_seconds(time_str)?;

    let total_minutes = total_seconds / 60.0;
    let pace_decimal = total_minutes / distance_km;

    let minutes = pace_decimal.floor() as u32;
    let seconds = ((pace_decimal - minutes as f32) * 60.0).round() as u32;

    Ok((minutes, seconds))
}

fn format_pace(minutes: u32, seconds: u32) -> String {
    format!("{}′{:02}″", minutes, seconds)
}
