use serde_json::Value;
use crate::storage::{load_transactions, save_content, };
use std::collections::HashMap;


pub fn extract_spin_combos() {
    let transactions: Vec<Value> = load_transactions("../data/hold_and_win/transactions/".to_string());
    let mut combos: Vec<HashMap<Vec<i64>, u64>> = Vec::new();
    for tr in &transactions {
        if let Some(reels) = tr["out"]["result"]["game"]["spins"][0]["spinData"]["reels"].as_array() {
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
                            for v in col_arr {
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

