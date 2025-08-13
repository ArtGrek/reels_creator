use serde_json::Value;
//use serde::{Deserialize, Serialize};
//use std::path::Path;
use std::{collections::HashMap, fs};
//use std::io::{self, Write};
//use std::collections::HashMap;
use ordered_float::NotNan;
//use indicatif::{ProgressBar, ProgressStyle};
mod models;
mod actions;
mod storage;
mod convert;
use models::{Game, Multi};
use actions::{extract, delete_boards_with_symbols, select_unique_boards, multiply_unique_boards_by_frequency, /*collect_reels, complete_reels, check_reels*/};
use storage::{load_transactions, save_debug, save_reels, save_rtp,};

use crate::bng::three_aztec_temples::models::Mode;
//use convert::{save_fugaso, fugaso_by_amount};

pub async fn execute(a_game_name: String, a_location: String) {
    //game config
    let game_config: Value = serde_json::from_str(&(fs::read_to_string("./".to_owned() + &a_game_name + ".json").unwrap_or_default())).unwrap_or_default();
    let width: usize = game_config.get("width").and_then(|v| v.as_i64()).unwrap_or(5) as usize;
    let height: usize = game_config.get("height").and_then(|v| v.as_i64()).unwrap_or(3) as usize;
    let spins_symbols: Vec<i64> = game_config.get("spins_symbols").and_then(|v| v.as_array()).unwrap_or(&vec![]).iter().filter_map(|v| v.as_i64()).collect();
    let bonus_symbols: Vec<i64> = game_config.get("bonus_symbols").and_then(|v| v.as_array()).unwrap_or(&vec![]).iter().filter_map(|v| v.as_i64()).collect();
    let appearing_symbols: Vec<i64> = game_config.get("appearing_symbols").and_then(|v| v.as_array()).unwrap_or(&vec![]).iter().filter_map(|v| v.as_i64()).collect();
    let bonus_symbol_values: Vec<Multi> = game_config.get("bonus_symbol_values").and_then(|v| v.as_array()).map(|arr| {arr.iter().filter_map(|v| {
        if let Some(i) = v.as_i64() {Some(Multi::Int(i))} else if let Some(f) = v.as_f64() {Some(Multi::Float(NotNan::new(f).unwrap()))} else if let Some(s) = v.as_str() {Some(Multi::String(s.to_string()))} else {None}
    }).collect()}).unwrap_or_default();
    let mysterty_symbol: i64 = game_config.get("mysterty_symbol").and_then(|v| v.as_i64()).unwrap_or(15);
    let buy_count: i64 = game_config.get("buy_count").and_then(|v| v.as_i64()).unwrap_or(2);
    let _can_skip_reel_collect_after_timeout: bool = game_config.get("can_skip_reel_collect_after_timeout").and_then(|v| v.as_bool()).unwrap_or(true);
    let _skip_collect_timeout_sec: i64 = game_config.get("skip_collect_timeout_sec").and_then(|v| v.as_i64()).unwrap_or(5);
    let _identical_complete: bool = game_config.get("identical_complete").and_then(|v| v.as_bool()).unwrap_or(true);
    //data
    let transactions: Vec<Value> = load_transactions(a_location.to_owned() + &a_game_name.clone() + "/transactions/");
    let mut game: Game = Game { 
        base: Mode { count: 0, spins: HashMap::new(), bonus: HashMap::new() }, 
        buy_1: Mode { count: 0, spins: HashMap::new(), bonus: HashMap::new() }, 
        buy_2: Mode { count: 0, spins: HashMap::new(), bonus: HashMap::new() } 
    };
    extract(&transactions, &spins_symbols, &appearing_symbols, &bonus_symbols, &bonus_symbol_values, mysterty_symbol, buy_count, width, height, &mut game);
    
    //actions
    //base
    for category_num in 0..categories.category.len() {
        for reel_num in 0..width {
            categories.category[category_num].boards.filtered[reel_num].instanses = delete_boards_with_symbols(categories.category[category_num].boards.total[reel_num].instanses.clone(), appearing_symbols.clone());
            categories.category[category_num].boards.unique[reel_num] = select_unique_boards(categories.category[category_num].boards.filtered[reel_num].instanses.clone());
            categories.category[category_num].boards.multiplied[reel_num].instanses = multiply_unique_boards_by_frequency(categories.category[category_num].boards.filtered[reel_num].instanses.clone(), &mut categories.category[category_num].boards.unique[reel_num], height);
            categories.category[category_num].boards.multiplied[reel_num].instanses.sort();
            //categories.category[category_num].reels[reel_num].instanses = collect_reels(&categories.category[category_num].boards.multiplied[reel_num].instanses, height as usize, can_skip_reel_collect_after_timeout, skip_collect_timeout_sec as u64);
            //complete_reels(&mut categories.category[category_num].reels[reel_num].instanses, identical_complete);
            //let boards = categories.category[category_num].boards.multiplied[reel_num].instanses.clone();
            //check_reels(&mut categories.category[category_num].reels[reel_num].instanses, &boards, height as usize);
        }
    }
    save_debug(&categories.category, &a_location, &a_game_name, "base");
    save_reels(categories.category.clone(), appearing_symbols.clone(), a_location.clone(), a_game_name.clone(), "", width, height);
    save_rtp(categories.category.clone(), a_location.clone(), a_game_name.clone(), "", width, height);
    //buy
    for i in 0..buy_count as usize {
        /*for category_num in 0..categories.buy_category[i].len() {
            for reel_num in 0..width {
                categories.buy_category[i][category_num].boards.filtered[reel_num].instanses = delete_boards_with_symbols(categories.buy_category[i][category_num].boards.total[reel_num].instanses.clone(), appearing_symbols.clone());
                categories.buy_category[i][category_num].boards.unique[reel_num] = select_unique_boards(categories.buy_category[i][category_num].boards.filtered[reel_num].instanses.clone());
                categories.buy_category[i][category_num].boards.multiplied[reel_num].instanses = multiply_unique_boards_by_frequency(categories.buy_category[i][category_num].boards.filtered[reel_num].instanses.clone(), &mut categories.buy_category[i][category_num].boards.unique[reel_num], height);
                categories.buy_category[i][category_num].boards.multiplied[reel_num].instanses.sort();
                categories.buy_category[i][category_num].reels[reel_num].instanses = collect_reels(&categories.buy_category[i][category_num].boards.multiplied[reel_num].instanses, height as usize, can_skip_reel_collect_after_timeout, skip_collect_timeout_sec as u64);
                complete_reels(&mut categories.buy_category[i][category_num].reels[reel_num].instanses, identical_complete);
                let boards = categories.buy_category[i][category_num].boards.multiplied[reel_num].instanses.clone();
                check_reels(&mut categories.buy_category[i][category_num].reels[reel_num].instanses, &boards, height as usize);
            }
        }*/
        save_debug(&categories.buy_category[i], &a_location, &a_game_name, &("buy".to_owned() + &i.to_string()));
        save_reels(categories.buy_category[i].clone(), appearing_symbols.clone(), a_location.clone(), a_game_name.clone(), &("_buy".to_owned() + &i.to_string()), width, height);
        save_rtp(categories.buy_category[i].clone(), a_location.clone(), a_game_name.clone(), &("_buy".to_owned() + &i.to_string()), width, height);
    }

    //save_fugaso(categories.clone(), appearing_symbols.clone(), a_location.clone(), a_game_name.clone());
    //fugaso_by_amount(&transactions, a_location.clone(), a_game_name.clone());
}