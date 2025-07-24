use tokio;
use serde_json::Value;
//use serde_json::from_str;
use std::fs;
use std::error::Error;
//use std::collections::HashSet;
use std::io::{self};
use std::io::Write;
//use std::ops::Index;
//use std::path::Path;
//use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
//use chrono::Local;
//use std::thread;
//use std::time::Duration;
mod bng;
mod netg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print!("\x1B[2J\x1B[1;1H"); io::stdout().flush().unwrap();
    let supported_games: Vec<String> = serde_json::from_str(&(fs::read_to_string("./supported_games.json".to_string()).unwrap_or_default())).unwrap_or_default();
    let supported_providers: Vec<String> = serde_json::from_str(&(fs::read_to_string("./supported_providers.json".to_string()).unwrap_or_default())).unwrap_or_default();
    let game_provider = loop {
        print!("Input game provider (required): "); let _ = io::Write::flush(&mut io::stdout()); let mut game_provider_input = String::new(); let _ = io::stdin().read_line(&mut game_provider_input);
        let trimmed = game_provider_input.trim().to_string();
        if !trimmed.is_empty() && supported_providers.contains(&trimmed) {break trimmed;}
    };
    let game_name = loop {
        print!("Input game name (required): "); let _ = io::Write::flush(&mut io::stdout()); let mut game_name_input = String::new(); let _ = io::stdin().read_line(&mut game_name_input);
        let trimmed = game_name_input.trim().to_string();
        if !trimmed.is_empty() && supported_games.contains(&trimmed) {break trimmed;}
    };
    //config
    let config: Value = serde_json::from_str(&(fs::read_to_string("./config.json".to_string()).unwrap_or_default())).unwrap_or_default();
    let location = config.get("location").and_then(|v| v.as_str()).unwrap_or("./");
    match game_provider.as_str() {
        "bng" => {bng::execute(game_name, location.to_string()).await}
        "netg" => {netg::execute(game_name, location.to_string()).await}
        _ => {eprintln!("\r\tProvider not implement");}
    }
    Ok(())
}