use serde_json::Value;
//use serde::{Deserialize, Serialize};
//use std::path::Path;
use std::fs;
//use std::io::{self, Write};
//use std::collections::HashMap;
//use indicatif::{ProgressBar, ProgressStyle};
mod models;
mod actions;
mod storage;
use models::Categories;
use actions::{_collect_reels, _multiply_unique_boards_by_frequency, check_reels, complete_reels, extract, split_by_frequency_average, delete_boards_with_symbols, select_unique_boards, };
use storage::{load_transactions, save_debug, save_reels,};
//use convert::{save_fugaso, fugaso_by_amount};

pub async fn execute(a_game_name: String, a_location: String) {
    //game config
    let game_config: Value = serde_json::from_str(&(fs::read_to_string("./".to_owned() + &a_game_name + ".json").unwrap_or_default())).unwrap_or_default();
    let width: usize = game_config.get("width").and_then(|v| v.as_i64()).unwrap_or(5) as usize;
    let height: usize = game_config.get("height").and_then(|v| v.as_i64()).unwrap_or(3) as usize;
    let appearing_symbols: Vec<i64> = game_config.get("appearing_symbols").and_then(|v| v.as_array()).unwrap_or(&vec![]).iter().filter_map(|v| v.as_i64()).collect();
    let can_skip_reel_collect_after_timeout: bool = game_config.get("can_skip_reel_collect_after_timeout").and_then(|v| v.as_bool()).unwrap_or(true);
    let skip_collect_timeout_sec: i64 = game_config.get("skip_collect_timeout_sec").and_then(|v| v.as_i64()).unwrap_or(5);
    let identical_complete: bool = game_config.get("identical_complete").and_then(|v| v.as_bool()).unwrap_or(true);
    //data
    let transactions: Vec<Value> = load_transactions(a_location.to_owned() + &a_game_name.clone() + "/transactions/");
    let mut categories: Categories = Categories { count: 0, category: Vec::new(), buy_category: vec![Vec::new(); 2], settings: Default::default() };
    extract(&transactions, width, height, &mut categories);
    for i in 0..width {
        categories.category[0].boards.filtered[i].instanses = delete_boards_with_symbols(categories.category[0].boards.total[i].instanses.clone(), &appearing_symbols);
        categories.category[0].boards.unique[i] = select_unique_boards(categories.category[0].boards.filtered[i].instanses.clone());
        categories.category[0].boards.unique[i].instanses.sort_by(|a, b| b.count.cmp(&a.count));
    }
    split_by_frequency_average(width, height, &mut categories);
    for category_num in 0..categories.category.len() {
        for reel_num in 0..width {
            //categories.category[category_num].boards.filtered[reel_num].instanses = delete_boards_with_symbols(categories.category[category_num].boards.total[reel_num].instanses.clone(), appearing_symbols.clone());
            //categories.category[category_num].boards.unique[reel_num] = select_unique_boards(categories.category[category_num].boards.filtered[reel_num].instanses.clone());
            //categories.category[category_num].boards.unique[reel_num].instanses.sort_by(|a, b| b.count.cmp(&a.count));
            categories.category[category_num].boards.multiplied[reel_num] = _multiply_unique_boards_by_frequency(categories.category[category_num].boards.filtered[reel_num].instanses.clone(), &mut categories.category[category_num].boards.unique[reel_num], height);
            categories.category[category_num].boards.multiplied[reel_num].instanses.sort_by(|a, b| b.count.cmp(&a.count));
            categories.category[category_num].reels[reel_num].instanses = _collect_reels(reel_num.to_string(), &categories.category[category_num].boards.multiplied[reel_num].instanses, height as usize, can_skip_reel_collect_after_timeout, skip_collect_timeout_sec as u64);
            complete_reels(&mut categories.category[category_num].reels[reel_num].instanses, identical_complete);
            let boards = categories.category[category_num].boards.multiplied[reel_num].instanses.clone();
            check_reels(&mut categories.category[category_num].reels[reel_num].instanses, &boards, height as usize);
        }
    }
    save_debug(&categories.category, &a_location, &a_game_name, "base");
    save_reels(categories.category.clone(), appearing_symbols.clone(), a_location.clone(), a_game_name.clone(), "", width, height);
}