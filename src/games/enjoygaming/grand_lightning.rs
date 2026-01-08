//src\games\enjoygaming\grand_lightning.rs
use serde_json::Value;
use crate::storage::{load_transactions, save_content, };
//use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle, };

pub fn extract_by_filter() {
    let mut filtred_transactions: Vec<Value> = Vec::new();
    let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/".to_string());
    let pb_main = ProgressBar::new((transactions.len()) as u64);
    pb_main.set_prefix("Find transactions wiht conditions: ");
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    pb_main.set_position(0);
    for transaction in &transactions {
        //start filter
        let multi_exist = transaction["out"]["context"]["spins"]["board"].as_array().map(|board_val| {
            board_val.iter().any(|board| {
                board.as_array().map(|col| {
                    col.iter().any(|sym| {
                        sym.as_i64() == Some(12)
                    })
                }).unwrap_or(false)
            })
        }).unwrap_or(false);
        if transaction["out"]["context"]["spins"]["bac_win"].as_bool().unwrap_or(false) && multi_exist {
            filtred_transactions.push(transaction.clone());
            break;
        }
        //end filter
        pb_main.inc(1);
    }
    pb_main.finish_with_message(" -> filtred ".to_owned() + &pb_main.position().to_string() + " transactions");
    let result = format!("\t\"filtred_transactions\": [\n{}\n\t]", filtred_transactions.iter().map(|transaction| {format!("\t\t{transaction}")}).collect::<Vec<String>>().join(",\n"));
    save_content("../data/enjoygaming/grand_lightning/settings/filtred.json".to_string(), format!("{{\n{}\n}}", result),);
}