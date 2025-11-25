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
                            if n >= 10 { valid = false; break; }
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
                    combo.iter().map(|n| {n.to_string()}).collect::<Vec<_>>().join(",")
                )
            }).collect::<Vec<String>>().join(",\n")
        )
    }).collect::<Vec<String>>().join(",\n");
    save_content("../data/hold_and_win/reels/spin_combos.json".to_string(), format!("{{\n\t\"reels\":[\n\t\t[\n{}\n\t\t]\n\t]\n}}", spin_combos),);
}

