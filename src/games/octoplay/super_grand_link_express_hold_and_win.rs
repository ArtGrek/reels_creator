use serde_json::Value;
use crate::storage::{load_transactions, save_content, };
use std::collections::HashMap;


pub fn extract_spin_combos() {
    let transactions: Vec<Value> = load_transactions("../data/hold_and_win/transactions/".to_string());

    let mut combos: Vec<HashMap<Vec<i64>, u64>> = Vec::new();

    for transaction in &transactions {
        if let Some(reels) = transaction["out"]["result"]["game"]["spins"][0]["spinData"]["reels"].as_array() {
            for (col_idx, col_val) in reels.iter().enumerate() {
                if let Some(col_arr) = col_val.as_array() {
                    let mut col_vec: Vec<i64> = Vec::with_capacity(3);
                    let mut valid = true;
                    for v in &col_arr[3..=5] {
                        if let Some(n) = v.as_i64() {
                            col_vec.push(n);
                        } else {valid = false; break;}
                    }
                    if !valid { continue; }
                    while combos.len() <= col_idx {combos.push(HashMap::new());}
                    let map = &mut combos[col_idx];
                    *map.entry(col_vec).or_insert(0) += 1;
                }
            }
        }
    }

    let spin_combos = combos.iter_mut().map(|col| {
        let total_col_count: u64 = col.values().sum();
        let mut col_count = 0;
        let mut combos_col: Vec<(&Vec<i64>, &u64)> = col.iter().collect();
        combos_col.sort_by(|(_, c1), (_, c2)| c1.cmp(c2));
        format!("\t\t\t{{\n{}\n\t\t\t}}", 
            combos_col.iter().map(|(combo, count)| {
                col_count += *count;
                format!("\t\t\t\t\"{}\": [{}]", 
                    (col_count as f64 * 100.0 / total_col_count as f64 * 10000.0) as i64, 
                    combo.iter().map(|n| {
                        let c = (b'A' + (*n as u8 - 1)) as char;
                        format!("\"{}\"", c)
                    }).collect::<Vec<_>>().join(",")
                )
            }).collect::<Vec<String>>().join(",\n")
        )
    }).collect::<Vec<String>>().join(",\n");

    save_content("../data/hold_and_win/reels/spin_combos.json".to_string(), format!("{{\n\t\"reels\":[\n\t\t[\n{}\n\t\t]\n\t]\n}}", spin_combos),);
}

pub fn extract_spin_over_bonus() {
    let transactions: Vec<Value> = load_transactions("../data/hold_and_win/transactions/".to_string());
    let mut total_transactions: u64 = 0;
    let mut matched_transactions: u64 = 0;
    let mut count_5: u64 = 0;
    let mut count_6: u64 = 0;
    let mut count_7: u64 = 0;
    let mut count_8: u64 = 0;
    let mut count_9: u64 = 0;
    let mut count_10: u64 = 0;
    let mut count_11: u64 = 0;
    let mut count_12: u64 = 0;
    let mut count_13: u64 = 0;
    let mut count_14: u64 = 0;
    let mut count_15: u64 = 0;
    for transaction in &transactions {
        total_transactions += 1;
        let spin = &transaction["out"]["result"]["game"]["spins"][0]["spinData"];
        if spin.get("activator").is_some() {
            if spin["reels"] != spin["reelsPayout"] {
                if let Some(reels_payout) = spin["reelsPayout"].as_array() {
                    matched_transactions += 1;
                    let mut gt_10_count: u64 = 0;
                    for col in reels_payout {
                        if let Some(col_arr) = col.as_array() {
                            for v in col_arr[3..=5].iter() {
                                if let Some(n) = v.as_i64() {
                                    if n >= 10 {gt_10_count += 1;}
                                }
                            }
                        }
                    }
                    match gt_10_count {
                        5 => count_5 += 1,
                        6 => count_6 += 1,
                        7 => count_7 += 1,
                        8 => count_8 += 1,
                        9 => count_9 += 1,
                        10 => count_10 += 1,
                        11 => count_11 += 1,
                        12 => count_12 += 1,
                        13 => count_13 += 1,
                        14 => count_14 += 1,
                        15 => count_15 += 1,
                        _ => {}
                    }
                }
            }
        }
    }

    let k0 = (total_transactions - matched_transactions) as f64 * 100.0 / total_transactions as f64 * 10000.0;
    let k5 = k0 + (count_5 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k6 = k5 + (count_6 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k7 = k6 + (count_7 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k8 = k7 + (count_8 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k9 = k8 + (count_9 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k10 = k9 + (count_10 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k11 = k10 + (count_11 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k12 = k11 + (count_12 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k13 = k12 + (count_13 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k14 = k13 + (count_14 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let k15 = k14 + (count_15 as f64 * 100.0 / total_transactions as f64 * 10000.0);
    let dist_over = format!(
        "\t\"distOver\": {{\n\
        \t\t\"{}\": 0,\n\
        \t\t\"{}\": 5,\n\
        \t\t\"{}\": 6,\n\
        \t\t\"{}\": 7,\n\
        \t\t\"{}\": 8,\n\
        \t\t\"{}\": 9\n\
        \t\t\"{}\": 10\n\
        \t\t\"{}\": 11\n\
        \t\t\"{}\": 12\n\
        \t\t\"{}\": 13\n\
        \t\t\"{}\": 14\n\
        \t\t\"{}\": 15\n\
        \t}}",
        k0 as i64, k5 as i64, k6 as i64, k7 as i64, k8 as i64, k9 as i64, k10 as i64, k11 as i64, k12 as i64, k13 as i64, k14 as i64, k15 as i64
    );
    save_content("../data/hold_and_win/reels/spin_over_bonus.json".to_string(), format!("{{\n{}\n}}", dist_over),);
}

pub fn extract_spin_coin_values() {
    let transactions: Vec<Value> = load_transactions("../data/hold_and_win/transactions/".to_string());
    let mut total_tiles: u64 = 0;
    let mut multiplier_counts: HashMap<i64, u64> = HashMap::new();
    for transaction in &transactions {
        if let Some(spins) = transaction["out"]["result"]["game"]["spins"].as_array() {
            for spin in spins {
                if spin["type"] == "freeSpin" {
                    if let Some(cash_tiles) = spin["spinData"]["cashTiles"].as_array() {
                        total_tiles += cash_tiles.len() as u64;
                        for tile in cash_tiles {
                            if tile["tileId"].as_i64() == Some(11) {
                                if let Some(multiplier_from) = tile["features"]["multiplier"]["from"].as_i64() {
                                    *multiplier_counts.entry(multiplier_from).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }



    let mut dist_coin: Vec<(i64, u64)> = multiplier_counts.iter()
        .map(|(&mult, &count)| (mult, count))
        .collect();
    dist_coin.sort_by_key(|&(mult, _)| mult);
    let mut accumulated: u64 = 0;
    let dist_coin_str = dist_coin.iter().map(|(mult, count)| {
        let key = (( *count as f64 * 100.0 / total_tiles as f64) * 10000.0) as u64;
        accumulated += key;
        format!("\t\t\"{}\": {}", accumulated, mult)
    }).collect::<Vec<_>>().join(",\n");
    let spin_coin_values = format!("\t\"distCoin\": {{\n{}\n\t}}", dist_coin_str);

    save_content("../data/hold_and_win/reels/spin_coin_values.json".to_string(), format!("{{\n{}\n}}", spin_coin_values),);
}

pub fn extract_respin_reels() {
    let transactions: Vec<Value> = load_transactions("../data/hold_and_win/transactions/".to_string());
    let mut cells_counts: HashMap<(usize, usize), HashMap<i64, u64>> = HashMap::new();
    let mut total_count: u64 = 0;
    for transaction in &transactions {
        if let Some(spins) = transaction["out"]["result"]["game"]["spins"].as_array() {
            for spin in spins {
                if spin["type"] == "freeSpin" {
                    if let Some(reels) = spin["spinData"]["reels"].as_array() {
                        total_count += 1;
                        for (x, col_val) in reels.iter().enumerate() {
                            if let Some(col_arr) = col_val.as_array() {
                                if x == 2 { 
                                    if let Some(reels_payout) = spin["spinData"]["reelsPayout"].as_array() {
                                        if let Some(col_arr_payout) = reels_payout[2].as_array() {
                                            if col_arr != col_arr_payout {
                                                let col: Vec<i64> = col_arr.iter().map(|v| v.as_i64().unwrap()).collect::<Vec<i64>>();
                                                let col_payout: Vec<i64> = col_arr_payout.iter().map(|v| v.as_i64().unwrap()).collect::<Vec<i64>>();
                                                for y in 0..col_payout.len() {
                                                    let map = cells_counts.entry((x, y)).or_insert_with(HashMap::new);
                                                    if col[y] != col_payout[y] {
                                                        *map.entry(col_payout[y]).or_insert(0) += 1;
                                                    } else {
                                                        *map.entry(16).or_insert(0) += 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    continue; 
                                }
                                for (y, cell) in col_arr[3..=5].iter().enumerate() {
                                    if let Some(sym) = cell.as_i64() {
                                        let map = cells_counts.entry((x, y)).or_insert_with(HashMap::new);
                                        *map.entry(sym).or_insert(0) += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut keys: Vec<(usize, usize)> = cells_counts.keys().cloned().collect();
    keys.sort_by_key(|&(x, y)| (x, y));


    let respin_reels = keys.iter().map(|&(x, y)| {
        let counts = &cells_counts[&(x, y)];
        let s = (10..=16).map(|sym| {
            let count = counts.get(&sym).cloned().unwrap_or(0);
            let n = (count as f64 * 100.0 / total_count as f64 * 1000.0) as usize;
            let c = (b'A' + (sym as u8 - 1)) as char;
            c.to_string().repeat(n)
        }).collect::<String>();
        format!("\t\t\"{}\"", s)
    }).collect::<Vec<String>>().join(",\n");


    save_content("../data/hold_and_win/reels/respin_reels.json".to_string(), format!("{{\n\t\"reels\":[\n{}\n\t]\n}}", respin_reels),);
}

pub fn extract_respin_coin_values() {
    let transactions: Vec<Value> = load_transactions("../data/hold_and_win/transactions/".to_string());
    let mut total_tiles: u64 = 0;
    let mut multiplier_counts: HashMap<i64, u64> = HashMap::new();
    for transaction in &transactions {
        if let Some(spins) = transaction["out"]["result"]["game"]["spins"].as_array() {
            for spin in spins {
                if spin["type"] == "freeSpin" {
                    if let Some(cash_tiles) = spin["spinData"]["cashTiles"].as_array() {
                        let mut have_11 = false;
                        for tile in cash_tiles {
                            if tile["tileId"].as_i64() == Some(11) {
                                have_11 = true;
                                if let Some(multiplier_from) = tile["features"]["multiplier"]["from"].as_i64() {
                                    *multiplier_counts.entry(multiplier_from).or_insert(0) += 1;
                                }
                            }
                        }
                        if have_11 {total_tiles += cash_tiles.len() as u64;}
                    }
                }
            }
        }
    }



    let mut dist_coin: Vec<(i64, u64)> = multiplier_counts.iter()
        .map(|(&mult, &count)| (mult, count))
        .collect();
    dist_coin.sort_by_key(|&(mult, _)| mult);
    let mut accumulated: u64 = 0;
    let dist_coin_str = dist_coin.iter().map(|(mult, count)| {
        let key = (( *count as f64 * 100.0 / total_tiles as f64) * 10000.0) as u64;
        accumulated += key;
        format!("\t\t\"{}\": {}", accumulated, mult)
    }).collect::<Vec<_>>().join(",\n");
    let respin_coin_values = format!("\t\"distCoin\": {{\n{}\n\t}}", dist_coin_str);

    save_content("../data/hold_and_win/reels/respin_coin_values.json".to_string(), format!("{{\n{}\n}}", respin_coin_values),);
}

