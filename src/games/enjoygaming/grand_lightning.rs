//src\games\enjoygaming\grand_lightning.rs
use serde_json::Value;
use crate::storage::{load_transactions, save_content, };
//use std::collections::HashMap;
use std::collections::HashSet;
use indicatif::{ProgressBar, ProgressStyle, };

#[test]fn test_extract_by_filter() {extract_by_filter();} pub fn extract_by_filter() {
    let mode = "bet_7000/";
    let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/".to_owned() + mode);
    //let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/bet_100/d881edc87b9a4faf8a8edd304943f635.json".to_string());
    let mut filtred_transactions: Vec<String> = Vec::new();
    let pb_main = ProgressBar::new((transactions.len()) as u64);
    pb_main.set_prefix("Find transactions wiht conditions: ");
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    // start filters

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

    // filter spin_overlay_details
    {
        pb_main.set_position(0);
        let mut find_count = 5;
        let mut spin_overlay_less: Vec<Value> = Vec::new();
        let mut spin_exists_min_value: Value = Default::default();
        let mut spin_exists_max_value: Value = Default::default();
        let mut spin_overlay_max_value: Value = Default::default();
        let mut spin_exists_min = 15usize;
        let mut spin_exists_max = 0usize;
        let mut spin_overlay_max = 0usize;
        for transaction in &transactions {

            let is_spin = transaction["in"]["action"]["name"].as_str() == Some("spin");
            let bac_win = transaction["out"]["context"]["spins"]["bac_win"].as_bool().unwrap_or(false);

            let spetials_overlaied = transaction["out"]["context"]["spins"]["board"].as_array().map(|board| {
                board.iter()
                    .flat_map(|col_val| col_val.as_array().into_iter().flatten())
                    .filter(|symbol| {
                        matches!(symbol.as_i64(), Some(10 | 11 | 12))
                    }).count()
            }).unwrap_or(0);

            let spetials_exist = transaction["out"]["context"]["spins"]["original_board"].as_array().map(|board| {
                board.iter()
                    .flat_map(|col_val| col_val.as_array().into_iter().flatten())
                    .filter(|symbol| {
                        matches!(symbol.as_i64(), Some(10 | 11 | 12))
                    }).count()
            }).unwrap_or(15);

            if bac_win {
                if spin_exists_min > spetials_exist {spin_exists_min = spetials_exist; spin_exists_min_value = transaction.clone()}
                if spin_exists_max < spetials_exist {spin_exists_max = spetials_exist; spin_exists_max_value = transaction.clone()}
                if spin_overlay_max < spetials_overlaied {spin_overlay_max = spetials_overlaied; spin_overlay_max_value = transaction.clone()}
            }


            if spetials_exist < 2 && spetials_overlaied > 5 && is_spin {
                find_count -= 1;
                spin_overlay_less.push(transaction.clone());
                if find_count <= 0 {break};
            }
            pb_main.inc(1);
        }
        filtred_transactions.push(format!("\t\t\"spin_exists_min\": {},\n\t\t\"spin_exists_min_value\": {},\n\t\t\"spin_exists_max\": {},\n\t\t\"spin_exists_max_value\": {},\n\t\t\"spin_overlay_max\": {},\n\t\t\"spin_overlay_max_value\": {},\n\t\t\"spin_overlay_less\":[\n{}\n\t\t]", 
            spin_exists_min,
            spin_exists_min_value,
            spin_exists_max,
            spin_exists_max_value,
            spin_overlay_max,
            spin_overlay_max_value,
            spin_overlay_less.iter().map(|transaction| {format!("\t\t\t{transaction}")}).collect::<Vec<String>>().join(",\n")
        ));
    }
    
    // end filter
    pb_main.finish_with_message(" -> finished");
    let result = format!("\t\"filtred_transactions\": {{\n{}\n\t}}", filtred_transactions.iter().map(|transaction| {format!("{transaction}")}).collect::<Vec<String>>().join(",\n"));
    save_content(format!("../data/enjoygaming/grand_lightning/temporary/{mode}filtred.json"), format!("{{\n{}\n}}", result),);
}


#[test]fn test_extract_coin_values() {extract_coin_values();} pub fn extract_coin_values() {
    let mode = "bet_30000/";
    let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/".to_owned() + mode);
    //let transactions: Vec<Value> = load_transactions("../data/enjoygaming/grand_lightning/transactions/bet_100/d881edc87b9a4faf8a8edd304943f635.json".to_string());
    let result: String;
    let pb_main = ProgressBar::new((transactions.len()) as u64);
    pb_main.set_prefix("Find transactions wiht conditions: ");
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    // start filters
    
    // filter coin_values
    {
        pb_main.set_position(0);
        let mut coin_values_set: HashSet<i32> = HashSet::new();
        for transaction in &transactions {
            let spins_board = &transaction["out"]["context"]["spins"]["board"];

            // 1) spins.bs_values + spins.board
            let spins_bs = &transaction["out"]["context"]["spins"]["bs_values"];
            collect_coin_values_from(spins_bs, spins_board, &mut coin_values_set);

            // 2) bonus.bs_values + bonus.board (если есть), иначе используем spins.board
            let bonus_bs = &transaction["out"]["context"]["bonus"]["bs_values"];
            let bonus_board = &transaction["out"]["context"]["bonus"]["board"];
            if bonus_board.is_array() {
                collect_coin_values_from(bonus_bs, bonus_board, &mut coin_values_set);
            } else {
                collect_coin_values_from(bonus_bs, spins_board, &mut coin_values_set);
            }
            pb_main.inc(1);
        }
        let mut coin_values: Vec<i32> = coin_values_set.into_iter().collect();
        coin_values.sort_unstable();
        result = format!("\t\"coin_values\":[\n{}\n\t]", coin_values.iter().map(|value| {format!("\t\t{value}")}).collect::<Vec<String>>().join(",\n"));
    }
    
    // end filter
    pb_main.finish_with_message(" -> finished");
    save_content(format!("../data/enjoygaming/grand_lightning/settings/{mode}coin_values.json"), format!("{{\n{}\n}}", result),);
}

fn collect_coin_values_from(bs_values: &Value, board: &Value, out: &mut HashSet<i32>) {
    let bs_cols = match bs_values.as_array() { Some(v) => v, None => return };
    let brd_cols = match board.as_array() { Some(v) => v, None => return };

    for (col_i, bs_col_v) in bs_cols.iter().enumerate() {
        let bs_col = match bs_col_v.as_array() { Some(v) => v, None => continue };
        let brd_col_v = match brd_cols.get(col_i) { Some(v) => v, None => continue };
        let brd_col = match brd_col_v.as_array() { Some(v) => v, None => continue };

        for (row_i, bs_cell) in bs_col.iter().enumerate() {
            let bs = match bs_cell.as_i64() { Some(v) => v, None => continue };
            if bs == 0 { continue; }

            let brd = brd_col.get(row_i).and_then(|v| v.as_i64()).unwrap_or(12);
            if brd != 12 {
                // bs_values у тебя целые, берём i32
                out.insert(bs as i32);
            }
        }
    }
}