//src\games\enjoygaming\grand_lightning.rs
use serde_json::Value;
use crate::storage::{load_transactions, save_content, };
//use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle, };

#[test]fn test_extract_by_filter() {extract_by_filter();} pub fn extract_by_filter() {
    let mut filtred_transactions: Vec<String> = Vec::new();
    //let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/".to_string());
    let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/bet_100/".to_string());
    //let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/bet_100/d881edc87b9a4faf8a8edd304943f635.json".to_string());
    let pb_main = ProgressBar::new((transactions.len()) as u64);
    pb_main.set_prefix("Find transactions wiht conditions: ");
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    // filter bac_win_and_multi_2_in_spin
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut bac_win_and_multi_2_in_spin: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let multi_exist = transaction["out"]["context"]["spins"]["board"].as_array().map(|board_val| {
                board_val.iter().enumerate().any(|(col_num, board)| {
                    board.as_array().map(|col| {
                        col.iter().enumerate().any(|(row_num, sym)| {
                            sym.as_i64() == Some(12) 
                            && transaction["out"]["context"]["spins"]["bs_values"].as_array()
                                .and_then(|bs| bs.get(col_num))
                                .and_then(|bs_col| bs_col.as_array())
                                .and_then(|bs_col| bs_col.get(row_num))
                                .and_then(|v| v.as_i64())
                                .map_or(false, |v| v == 2)
                        })
                    }).unwrap_or(false)
                })
            }).unwrap_or(false);
            if multi_exist 
            && transaction["out"]["context"]["spins"]["bac_win"].as_bool().unwrap_or(false) 
            && transaction["in"]["action"]["name"].as_str() == Some("spin") {
                find_count -= 1;
                bac_win_and_multi_2_in_spin.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"bac_win_and_multi_2_in_spin\":[\n{}\n\t\t]", 
            bac_win_and_multi_2_in_spin.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter bac_win_and_multi_3_in_spin
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut bac_win_and_multi_3_in_spin: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let multi_exist = transaction["out"]["context"]["spins"]["board"].as_array().map(|board_val| {
                board_val.iter().enumerate().any(|(col_num, board)| {
                    board.as_array().map(|col| {
                        col.iter().enumerate().any(|(row_num, sym)| {
                            sym.as_i64() == Some(12)
                            &&  transaction["out"]["context"]["spins"]["bs_values"].as_array()
                                .and_then(|bs| bs.get(col_num))
                                .and_then(|bs_col| bs_col.as_array())
                                .and_then(|bs_col| bs_col.get(row_num))
                                .and_then(|v| v.as_i64())
                                .map_or(false, |v| v == 3)
                        })
                    }).unwrap_or(false)
                })
            }).unwrap_or(false);
            if multi_exist 
            && transaction["out"]["context"]["spins"]["bac_win"].as_bool().unwrap_or(false) 
            && transaction["in"]["action"]["name"].as_str() == Some("spin") {
                find_count -= 1;
                bac_win_and_multi_3_in_spin.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"bac_win_and_multi_3_in_spin\":[\n{}\n\t\t]", 
            bac_win_and_multi_3_in_spin.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter bac_win_and_multi_5_in_spin
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut bac_win_and_multi_5_in_spin: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let multi_exist = transaction["out"]["context"]["spins"]["board"].as_array().map(|board_val| {
                board_val.iter().enumerate().any(|(col_num, board)| {
                    board.as_array().map(|col| {
                        col.iter().enumerate().any(|(row_num, sym)| {
                            sym.as_i64() == Some(12)
                            &&  transaction["out"]["context"]["spins"]["bs_values"].as_array()
                                .and_then(|bs| bs.get(col_num))
                                .and_then(|bs_col| bs_col.as_array())
                                .and_then(|bs_col| bs_col.get(row_num))
                                .and_then(|v| v.as_i64())
                                .map_or(false, |v| v == 5)
                        })
                    }).unwrap_or(false)
                })
            }).unwrap_or(false);
            if multi_exist 
            && transaction["out"]["context"]["spins"]["bac_win"].as_bool().unwrap_or(false) 
            && transaction["in"]["action"]["name"].as_str() == Some("spin") {
                find_count -= 1;
                bac_win_and_multi_5_in_spin.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"bac_win_and_multi_5_in_spin\":[\n{}\n\t\t]", 
            bac_win_and_multi_5_in_spin.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter double_multi_in_spin
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut double_multi_in_spin: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let double_multi_exist = transaction["out"]["context"]["spins"]["board"].as_array().map(|board_val| {
                board_val.iter().any(|board| {
                    board.as_array().map(|cols| {
                        cols.iter()
                            .flat_map(|col| col.as_array().into_iter().flatten())
                            .filter(|sym| sym.as_i64() == Some(12))
                            .take(2)
                            .count() > 1
                    }).unwrap_or(false)
                })
            }).unwrap_or(false);
            if double_multi_exist 
            && transaction["in"]["action"]["name"].as_str() == Some("spin") {
                find_count -= 1;
                double_multi_in_spin.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"double_multi_in_spin\":[\n{}\n\t\t]", 
            double_multi_in_spin.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter double_multi_in_bonus
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut double_multi_in_bonus: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let double_multi_exist = transaction["out"]["context"]["bonus"]["changes"].as_array().map(|changes| {
                changes.iter().filter(|change| {
                    change["symbol"].as_i64() == Some(12) 
                }).count() > 1
            }).unwrap_or(false);
            if double_multi_exist {
                find_count -= 1;
                double_multi_in_bonus.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"double_multi_in_bonus\":[\n{}\n\t\t]", 
            double_multi_in_bonus.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter multi_2_in_bonus
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut multi_2_in_bonus: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let multi_exist = transaction["out"]["context"]["bonus"]["changes"].as_array().map(|changes| {
                changes.iter().any(|change| {
                    change.as_object().map(|new| {
                        new["symbol"].as_i64() == Some(12) &&  new["multiplier"].as_i64() == Some(2)
                    }).unwrap_or(false)
                })
            }).unwrap_or(false);
            if multi_exist 
            && transaction["in"]["action"]["name"].as_str() == Some("respin") {
                find_count -= 1;
                multi_2_in_bonus.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"multi_2_in_bonus\":[\n{}\n\t\t]", 
            multi_2_in_bonus.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter multi_3_in_bonus
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut multi_3_in_bonus: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let multi_exist = transaction["out"]["context"]["bonus"]["changes"].as_array().map(|changes| {
                changes.iter().any(|change| {
                    change.as_object().map(|new| {
                        new["symbol"].as_i64() == Some(12) &&  new["multiplier"].as_i64() == Some(3)
                    }).unwrap_or(false)
                })
            }).unwrap_or(false);
            if multi_exist 
            && transaction["in"]["action"]["name"].as_str() == Some("respin") {
                find_count -= 1;
                multi_3_in_bonus.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"multi_3_in_bonus\":[\n{}\n\t\t]", 
            multi_3_in_bonus.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter multi_5_in_bonus
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut multi_5_in_bonus: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let multi_exist = transaction["out"]["context"]["bonus"]["changes"].as_array().map(|changes| {
                changes.iter().any(|change| {
                    change.as_object().map(|new| {
                        new["symbol"].as_i64() == Some(12) &&  new["multiplier"].as_i64() == Some(5)
                    }).unwrap_or(false)
                })
            }).unwrap_or(false);
            if multi_exist 
            && transaction["in"]["action"]["name"].as_str() == Some("respin") {
                find_count -= 1;
                multi_5_in_bonus.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"multi_5_in_bonus\":[\n{}\n\t\t]", 
            multi_5_in_bonus.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter grand_win_in_bonus
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut grand_win_in_bonus: Vec<Value> = Vec::new();
        for transaction in &transactions {
            let grand_win = transaction["out"]["context"]["bonus"]["grand"].as_array().map(|grand| {
                grand.iter().all(|v| v.as_i64() != Some(0))
            }).unwrap_or(false);
            if grand_win 
            && transaction["out"]["context"]["bonus"]["changes"].as_array().is_some_and(|a| !a.is_empty())
            && transaction["in"]["action"]["name"].as_str() == Some("respin") {
                find_count -= 1;
                grand_win_in_bonus.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"grand_win_in_bonus\":[\n{}\n\t\t]", 
            grand_win_in_bonus.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }

    // filter original_board_in_bonus
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut original_board_in_bonus: Vec<Value> = Vec::new();
        for transaction in &transactions {
            if transaction["out"]["context"]["bonus"]["original_board"].is_array() {
                find_count -= 1;
                original_board_in_bonus.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"original_board_in_bonus\":[\n{}\n\t\t]", 
            original_board_in_bonus.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }
    
    // end filter
    pb_main.finish_with_message(" -> finished");
    let result = format!("\t\"filtred_transactions\": {{\n{}\n\t}}", filtred_transactions.iter().map(|transaction| {format!("{transaction}")}).collect::<Vec<String>>().join(",\n"));
    save_content("../data/enjoygaming/grand_lightning/temporary/filtred.json".to_string(), format!("{{\n{}\n}}", result),);
}