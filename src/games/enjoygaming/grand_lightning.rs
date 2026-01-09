//src\games\enjoygaming\grand_lightning.rs
use serde_json::Value;
use crate::storage::{load_transactions, save_content, };
//use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle, };

#[test]fn test_extract_by_filter() {extract_by_filter();} pub fn extract_by_filter() {
    let mut filtred_transactions: Vec<String> = Vec::new();
    let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/".to_string());
    let pb_main = ProgressBar::new((transactions.len()) as u64);
    pb_main.set_prefix("Find transactions wiht conditions: ");
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    //first filter
    pb_main.set_position(0);
    let mut find_count = 5;
    let mut bac_win_and_multi_in_spin: Vec<Value> = Vec::new();
    for transaction in &transactions {
        let multi_exist = transaction["out"]["context"]["spins"]["board"].as_array().map(|board_val| {
            board_val.iter().any(|board| {
                board.as_array().map(|col| {
                    col.iter().any(|sym| {
                        sym.as_i64() == Some(12)
                    })
                }).unwrap_or(false)
            })
        }).unwrap_or(false);
        if multi_exist 
        && transaction["out"]["context"]["spins"]["bac_win"].as_bool().unwrap_or(false) 
        && transaction["in"]["action"]["name"].as_str() == Some("spin") {
            find_count -= 1;
            bac_win_and_multi_in_spin.push(transaction.clone());
            if find_count <= 0 {break};
        }
        pb_main.inc(1);
    }
    filtred_transactions.push(format!("\t\t\"bac_win_and_multi_in_spin\":[\n{}\n\t\t]", bac_win_and_multi_in_spin.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")));
    //second filter
    pb_main.set_position(0);
    let mut multi_in_bonus: Vec<Value> = Vec::new();
    let mut find_count = 5;
    for transaction in &transactions {
        let multi_exist = transaction["out"]["context"]["bonus"]["board"].as_array().map(|board_val| {
            board_val.iter().any(|board| {
                board.as_array().map(|col| {
                    col.iter().any(|sym| {
                        sym.as_i64() == Some(12)
                    })
                }).unwrap_or(false)
            })
        }).unwrap_or(false);
        if multi_exist {
            find_count -= 1;
            multi_in_bonus.push(transaction.clone());
            if find_count <= 0 {break};
        }
        pb_main.inc(1);
    }
    filtred_transactions.push(format!("\t\t\"multi_in_bonus\":[\n{}\n\t\t]", multi_in_bonus.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")));
    // end filter
    pb_main.finish_with_message(" -> finished");
    let result = format!("\t\"filtred_transactions\": {{\n{}\n\t}}", filtred_transactions.iter().map(|transaction| {format!("{transaction}")}).collect::<Vec<String>>().join(",\n"));
    save_content("../data/enjoygaming/grand_lightning/settings/filtred.json".to_string(), format!("{{\n{}\n}}", result),);
}