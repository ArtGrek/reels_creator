use serde_json::Value;
use crate::storage::{load_transactions, save_content, };

pub fn extract_spin_coins() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;
    let mut combos = vec![[0usize; 1 << 5]; 5];
    let lines: [[usize; 5]; 5] = [
        [0, 5, 10, 15, 20],
        [1, 6, 11, 16, 21],
        [2, 7, 12, 17, 22],
        [3, 8, 13, 18, 23],
        [4, 9, 14, 19, 24],
    ];
    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn == "reveal" {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (line_idx, positions) in lines.iter().enumerate() {
                                let mut mask: u8 = 0;
                                for (bit, &i) in positions.iter().enumerate() {
                                    if chars[i] != '(' {mask |= 1 << bit;}
                                }
                                combos[line_idx][mask as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total grids count: {}", grids_count);
    let coins = combos.iter().map(|combo| {
        let mut posibility = 0;
        let col = (0..(1usize << 5)).rev().map(|m|  {
            let pattern = (0..5).map(|b| {if (m >> b) & 1 == 1 { "\"@\"" } else { "\"O\"" }}).collect::<Vec<&str>>().join(",");
            if combo[m] > 0 {
                //posibility += combo[m];
                posibility += (combo[m] as f64 * 100.0 / grids_count as f64 * 10000.0) as i64 ;
                format!("\t\t\t\t\"{}\": [{}]", posibility, pattern)
            } else {format!("\t\t\t\t\"{}\": [{}]", combo[m], pattern)}
        }).collect::<Vec<String>>().join(",\n");
        format!("\t\t\t{{\n{}\n\t\t\t}}", col)
    }).collect::<Vec<_>>().join(",\n");
    save_content("../data/gladius_death_or_glory/reels/spin_coins.json".to_string(), format!("{{\n\t\"reels\":[\n\t\t[\n{}\n\t\t]\n\t]\n}}", coins));
}


pub fn extract_spin_coin_cell() {
    let transactions: Vec<Value> = load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;
    let mut count = 0usize;
    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            for ev in events {
                if ev["etn"] == "reveal" {
                    if let Some(grid) = ev["c"]["grid"].as_str() {
                        let chars: Vec<char> = grid.chars().skip(2).collect();
                        count += chars.iter().filter(|&&c| c == '(').count();
                        grids_count += 1;
                    }
                }
            }
        }
    }
    println!("grids count: {} count: {}", grids_count, count);
    let coin_posibility = (count as f64 * 100.0 / grids_count as f64 / 25.0 * 10000.0) as u64;
    let result = format!("{{\n\t\"coin\": {{\n\t\t\"{coin_posibility}\": \"O\"\n\t}}\n}}");
    save_content("../data/gladius_death_or_glory/reels/spin_coin_cell.json".to_string(), result,);
}


pub fn extract_spin_coin_values() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;

    let mut counts = [0usize; 12]; 
    let equiv = ['K','L','M','N','G','H','I','J','Q','R','S','T'];

    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            let mut fined_symbols = Vec::new();
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("coin_reveal_") {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (i,ch) in chars.iter().enumerate() {
                                if !fined_symbols.contains(&i) {
                                    match ch {
                                        '=' => {fined_symbols.push(i); counts[0] += 1}, // K
                                        '>' => {fined_symbols.push(i); counts[1] += 1}, // L
                                        '?' => {fined_symbols.push(i); counts[2] += 1}, // M
                                        '@' => {fined_symbols.push(i); counts[3] += 1}, // N
                                        'G' => {fined_symbols.push(i); counts[4] += 1}, // G
                                        'H' => {fined_symbols.push(i); counts[5] += 1}, // H
                                        'I' => {fined_symbols.push(i); counts[6] += 1}, // I
                                        'J' => {fined_symbols.push(i); counts[7] += 1}, // J
                                        'Q' => {fined_symbols.push(i); counts[8] += 1}, // Q
                                        'R' => {fined_symbols.push(i); counts[9] += 1}, // R
                                        'S' => {fined_symbols.push(i); counts[10] += 1}, // S
                                        'T' => {fined_symbols.push(i); counts[11] += 1}, // T
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total grids count: {}", grids_count);
    let coins_count: usize = counts.iter().sum();
    println!("total coins count: {}", coins_count);
    
    let mut posibility = 0;
    let coin_values = (0..equiv.len()).map(|i| {
        if counts[i] > 0 {
            //posibility += counts[i];
            posibility += (counts[i] as f64 * 100.0 / coins_count as f64 * 10000.0) as i64 ;
            format!("\t\t\"{}\": \"{}\"", posibility, equiv[i])
        } else {format!("\t\t\"{}\": \"{}\"", counts[i], equiv[i])}
    }).collect::<Vec<_>>().join(",\n");
    save_content("../data/gladius_death_or_glory/reels/spin_coin_values.json".to_string(), format!("{{\n\t\"coin_values\":{{\n{}\n\t}}\n}}", coin_values));
}


pub fn _extract_spin_coin_values() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;

    let mut counts = [0usize; 19]; 
    let equiv = ['K','L','M','N','G','H','I','J','Q','R','S','T','E','U','V','W','X','Y','Z'];

    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            let mut fined_symbols = Vec::new();
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("coin_reveal_") {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (i,ch) in chars.iter().enumerate() {
                                if !fined_symbols.contains(&i) {
                                    match ch {
                                        '=' => {fined_symbols.push(i); counts[0] += 1}, // K
                                        '>' => {fined_symbols.push(i); counts[1] += 1}, // L
                                        '?' => {fined_symbols.push(i); counts[2] += 1}, // M
                                        '@' => {fined_symbols.push(i); counts[3] += 1}, // N
                                        'G' => {fined_symbols.push(i); counts[4] += 1}, // G
                                        'H' => {fined_symbols.push(i); counts[5] += 1}, // H
                                        'I' => {fined_symbols.push(i); counts[6] += 1}, // I
                                        'J' => {fined_symbols.push(i); counts[7] += 1}, // J
                                        'Q' => {fined_symbols.push(i); counts[8] += 1}, // Q
                                        'R' => {fined_symbols.push(i); counts[9] += 1}, // R
                                        'S' => {fined_symbols.push(i); counts[10] += 1}, // S
                                        'T' => {fined_symbols.push(i); counts[11] += 1}, // T
                                        
                                        ')' => {fined_symbols.push(i); counts[12] += 1}, // E
                                        
                                        '+' => {fined_symbols.push(i); counts[13] += 1}, // U
                                        ',' => {fined_symbols.push(i); counts[14] += 1}, // V
                                        '-' => {fined_symbols.push(i); counts[15] += 1}, // W
                                        '.' => {fined_symbols.push(i); counts[16] += 1}, // X
                                        '/' => {fined_symbols.push(i); counts[17] += 1}, // Y
                                        '0' => {fined_symbols.push(i); counts[18] += 1}, // Z
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total grids count: {}", grids_count);
    let coins_count: usize = counts.iter().sum();
    println!("total coins count: {}", coins_count);
    
    let mut posibility = 0;
    let coin_values = (0..equiv.len()).map(|i| {
        if counts[i] > 0 {
            //posibility += counts[i];
            posibility += (counts[i] as f64 * 100.0 / coins_count as f64 * 10000.0) as i64 ;
            format!("\t\t\"{}\": \"{}\"", posibility, equiv[i])
        } else {format!("\t\t\"{}\": \"{}\"", counts[i], equiv[i])}
    }).collect::<Vec<_>>().join(",\n");
    save_content("../data/gladius_death_or_glory/reels/spin_coin_values.json".to_string(), format!("{{\n\t\"coin_values\":{{\n{}\n\t}}\n}}", coin_values));
}


pub fn extract_spin_bonus() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;
    let mut count = 0usize; 
    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn == "reveal" {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();

                            for ch in chars {
                                match ch {
                                    '3' => count += 1, // P
                                    _ => {}
                                }
                            }

                        }
                    }
                }
            }
        }
    }
    println!("total grids count: {}", grids_count);
    println!("total bonus count: {}", count);
    let posibility = (count as f64 * 100.0 / grids_count as f64 / 25.0 * 10000.0) as i64 ;
    save_content("../data/gladius_death_or_glory/reels/spin_bonus.json".to_string(), format!("{{\n\t\"bunus\":{{\n\t\t\"{}\":\"P\",\n\t\t\"999999\":\"@\"\n\t}}\n}}", posibility));
}


pub fn extract_spin_collector() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;
    let mut count = 0usize; 
    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            let mut fined_symbols = Vec::new();
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("coin_reveal_") {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (i,ch) in chars.iter().enumerate() {
                                if !fined_symbols.contains(&i) {
                                    match ch {
                                        ')' => {
                                            fined_symbols.push(i);
                                            count += 1
                                        }, // E
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total grids count: {}", grids_count);
    println!("total collector count: {}", count);
    let posibility = (count as f64 * 100.0 / grids_count as f64 / 25.0 * 10000.0) as i64 ;
    save_content("../data/gladius_death_or_glory/reels/spin_collector.json".to_string(), format!("{{\n\t\"collector\":{{\n\t\t\"{}\":\"E\",\n\t\t\"999999\":\"@\"\n\t}}\n}}", posibility));
}


pub fn extract_spin_multypliers() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;

    let mut counts = [0usize; 12]; 
    let equiv = ['U','V','W','X','Y','Z'];

    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            let mut fined_symbols = Vec::new();
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("coin_reveal_") {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (i,ch) in chars.iter().enumerate() {
                                if !fined_symbols.contains(&i) {
                                    match ch {
                                        '+' => {fined_symbols.push(i); counts[0] += 1}, // U
                                        ',' => {fined_symbols.push(i); counts[1] += 1}, // V
                                        '-' => {fined_symbols.push(i); counts[2] += 1}, // W
                                        '.' => {fined_symbols.push(i); counts[3] += 1}, // X
                                        '/' => {fined_symbols.push(i); counts[4] += 1}, // Y
                                        '0' => {fined_symbols.push(i); counts[5] += 1}, // Z
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total grids count: {}", grids_count);
    let coins_count: usize = counts.iter().sum();
    println!("total multyplier count: {}", coins_count);
    
    let mut posibility = 0;
    let coin_values = (0..equiv.len()).map(|i| {
        if counts[i] > 0 {
            //posibility += counts[i];
            posibility += (counts[i] as f64 * 100.0 / grids_count as f64 * 10000.0) as i64 ;
            format!("\t\t\"{}\": \"{}\"", posibility, equiv[i])
        } else {format!("\t\t\"{}\": \"{}\"", counts[i], equiv[i])}
    }).collect::<Vec<_>>().join(",\n");
    save_content("../data/gladius_death_or_glory/reels/spin_multypliers.json".to_string(), format!("{{\n\t\"multyplier\":{{\n{},\n\t\t\"999999\":\"@\"\n\t}}\n}}", coin_values));
}


pub fn extract_fs_spin_coins() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;
    let mut combos = vec![[0usize; 1 << 5]; 5];
    let lines: [[usize; 5]; 5] = [
        [0, 5, 10, 15, 20],
        [1, 6, 11, 16, 21],
        [2, 7, 12, 17, 22],
        [3, 8, 13, 18, 23],
        [4, 9, 14, 19, 24],
    ];
    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn == "fs_reveal" {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (line_idx, positions) in lines.iter().enumerate() {
                                let mut mask: u8 = 0;
                                for (bit, &i) in positions.iter().enumerate() {
                                    if chars[i] != '(' {mask |= 1 << bit;}
                                }
                                combos[line_idx][mask as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total grids count: {}", grids_count);
    let coins = combos.iter().map(|combo| {
        let mut posibility = 0;
        let col = (0..(1usize << 5)).rev().map(|m|  {
            let pattern = (0..5).map(|b| {if (m >> b) & 1 == 1 { "\"@\"" } else { "\"O\"" }}).collect::<Vec<&str>>().join(",");
            if combo[m] > 0 {
                //posibility += combo[m];
                posibility += (combo[m] as f64 * 100.0 / grids_count as f64 * 10000.0) as i64 ;
                format!("\t\t\t\t\"{}\": [{}]", posibility, pattern)
            } else {format!("\t\t\t\t\"{}\": [{}]", combo[m], pattern)}
        }).collect::<Vec<String>>().join(",\n");
        format!("\t\t\t{{\n{}\n\t\t\t}}", col)
    }).collect::<Vec<_>>().join(",\n");
    save_content("../data/gladius_death_or_glory/reels/fs_spin_coins.json".to_string(), format!("{{\n\t\"reels\":[\n\t\t[\n{}\n\t\t]\n\t]\n}}", coins));
}


pub fn extract_fs_spin_coin_cell() {
    let transactions: Vec<Value> = load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = vec![0usize; 25];
    let mut counts = vec![0usize; 25];
    for tx in &transactions {
        let mut step_index = 0usize;
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            for ev in events {
                if ev["etn"] == "fs_reveal" {
                    if let Some(grid) = ev["c"]["grid"].as_str() {
                        let chars: Vec<char> = grid.chars().skip(2).collect();
                        let count: usize = chars.iter().filter(|&&c| c == '(').count();
                        counts[step_index] += count-step_index;
                        grids_count[step_index] += 1;
                    }
                }
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("fs_coin_reveal_") {
                        if let Some(actions) = ev["c"]["actions"].as_array() {
                            for action in actions {
                                if action["at"] == "cashWin" {
                                    if let Some(h_str) = action["data"]["h"].as_str() {
                                        step_index = h_str.chars().filter(|&c| c == '1').count();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let result = format!("{{\n\t\"coin\": [{}\n\t]\n}}", 
        (0..25).map(|step_index| {
            println!("step: {} grids count: {} count: {}", step_index+1, grids_count[step_index], counts[step_index]);
            format!("\n\t\t{{\"{}\": \"O\"}}", (counts[step_index] as f64 * 100.0 / grids_count[step_index] as f64 / 25.0 * 10000.0) as u64)
        }).collect::<Vec<_>>().join(",")
    );
    save_content("../data/gladius_death_or_glory/reels/fs_spin_coin_cell.json".to_string(), result,);
}


pub fn extract_fs_spin_coin_values() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;

    let mut counts = [0usize; 19]; 
    let equiv = ['K','L','M','N','G','H','I','J','Q','R','S','T','E','U','V','W','X','Y','Z'];

    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            let mut fined_symbols = Vec::new();
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("fs_coin_reveal_") {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (i,ch) in chars.iter().enumerate() {
                                if !fined_symbols.contains(&i) {
                                    match ch {
                                        '=' => {fined_symbols.push(i); counts[0] += 1}, // K
                                        '>' => {fined_symbols.push(i); counts[1] += 1}, // L
                                        '?' => {fined_symbols.push(i); counts[2] += 1}, // M
                                        '@' => {fined_symbols.push(i); counts[3] += 1}, // N
                                        'G' => {fined_symbols.push(i); counts[4] += 1}, // G
                                        'H' => {fined_symbols.push(i); counts[5] += 1}, // H
                                        'I' => {fined_symbols.push(i); counts[6] += 1}, // I
                                        'J' => {fined_symbols.push(i); counts[7] += 1}, // J
                                        'Q' => {fined_symbols.push(i); counts[8] += 1}, // Q
                                        'R' => {fined_symbols.push(i); counts[9] += 1}, // R
                                        'S' => {fined_symbols.push(i); counts[10] += 1}, // S
                                        'T' => {fined_symbols.push(i); counts[11] += 1}, // T
                                        
                                        ')' => {fined_symbols.push(i); counts[12] += 1}, // E
                                        
                                        '+' => {fined_symbols.push(i); counts[13] += 1}, // U
                                        ',' => {fined_symbols.push(i); counts[14] += 1}, // V
                                        '-' => {fined_symbols.push(i); counts[15] += 1}, // W
                                        '.' => {fined_symbols.push(i); counts[16] += 1}, // X
                                        '/' => {fined_symbols.push(i); counts[17] += 1}, // Y
                                        '0' => {fined_symbols.push(i); counts[18] += 1}, // Z
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total grids count: {}", grids_count);
    let coins_count: usize = counts.iter().sum();
    println!("total coins count: {}", coins_count);
    
    let mut posibility = 0;
    let coin_values = (0..equiv.len()).map(|i| {
        if counts[i] > 0 {
            posibility += (counts[i] as f64 * 100.0 / coins_count as f64 * 10000.0) as i64 ;
            format!("\t\t\"{}\": \"{}\"", posibility, equiv[i])
        } else {format!("\t\t\"{}\": \"{}\"", counts[i], equiv[i])}
    }).collect::<Vec<_>>().join(",\n");
    save_content("../data/gladius_death_or_glory/reels/fs_spin_coin_values.json".to_string(), format!("{{\n\t\"coin_values\":{{\n{}\n\t}}\n}}", coin_values));
}


pub fn extract_fs_spin_collector() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = vec![0usize; 25];
    let mut counts = vec![0usize; 25];
    for tx in &transactions {
        let mut step_index = 0usize;
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            let mut fined_symbols = Vec::new();
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("fs_coin_reveal_") {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count[step_index] += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (i,ch) in chars.iter().enumerate() {
                                if !fined_symbols.contains(&i) {
                                    match ch {
                                        ')' => {
                                            fined_symbols.push(i);
                                            counts[step_index] += 1
                                        }, // E
                                        _ => {}
                                    }
                                }
                            }
                        }
                        if let Some(actions) = ev["c"]["actions"].as_array() {
                            for action in actions {
                                if action["at"] == "cashWin" {
                                    if let Some(h_str) = action["data"]["h"].as_str() {
                                        step_index = h_str.chars().filter(|&c| c == '1').count();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let result = format!("{{\n\t\"collector\": [{}\n\t]\n}}", 
        (0..25).map(|step_index| {
            println!("step: {} grids count: {} collector count: {}", step_index+1, grids_count[step_index], counts[step_index]);
            format!("\n\t\t{{\"{}\": \"O\",\"999999\":\"@\"}}", (counts[step_index] as f64 * 100.0 / grids_count[step_index] as f64 / 25.0 * 10000.0) as u64)
        }).collect::<Vec<_>>().join(",")
    );
    save_content("../data/gladius_death_or_glory/reels/fs_spin_collector.json".to_string(), result);
}


pub fn extract_fs_spin_multypliers() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = vec![0usize; 25];
    let mut counts = vec![[0usize; 12]; 25]; 
    let equiv = ['U','V','W','X','Y','Z'];

    for tx in &transactions {
        let mut step_index = 0usize;
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            let mut fined_symbols = Vec::new();
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("fs_coin_reveal_") {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count[step_index] += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();
                            for (i,ch) in chars.iter().enumerate() {
                                if !fined_symbols.contains(&i) {
                                    match ch {
                                        '+' => {fined_symbols.push(i); counts[step_index][0] += 1}, // U
                                        ',' => {fined_symbols.push(i); counts[step_index][1] += 1}, // V
                                        '-' => {fined_symbols.push(i); counts[step_index][2] += 1}, // W
                                        '.' => {fined_symbols.push(i); counts[step_index][3] += 1}, // X
                                        '/' => {fined_symbols.push(i); counts[step_index][4] += 1}, // Y
                                        '0' => {fined_symbols.push(i); counts[step_index][5] += 1}, // Z
                                        _ => {}
                                    }
                                }
                            }
                        }
                        if let Some(actions) = ev["c"]["actions"].as_array() {
                            for action in actions {
                                if action["at"] == "cashWin" {
                                    if let Some(h_str) = action["data"]["h"].as_str() {
                                        step_index = h_str.chars().filter(|&c| c == '1').count();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let result = format!("{{\n\t\"multyplier\": [{}\n\t]\n}}", 
        (0..25).map(|step_index| {
            let coins_count: usize = counts[step_index].iter().sum();
            println!("step: {} grids count: {} collector count: {}", step_index+1, grids_count[step_index], coins_count);
            let mut posibility = 0;
            format!("\n\t\t{{{},\n\t\t\t\"999999\":\"@\"\n\t\t}}", 
                (0..equiv.len()).map(|i| {
                    if counts[step_index][i] > 0 {
                        posibility += (counts[step_index][i] as f64 * 100.0 / grids_count[step_index] as f64 * 10000.0) as i64 ;
                        format!("\n\t\t\t\"{}\": \"{}\"", posibility, equiv[i])
                    } else {format!("\n\t\t\t\"{}\": \"{}\"", counts[step_index][i], equiv[i])}
                }).collect::<Vec<_>>().join(",")
            )
        }).collect::<Vec<_>>().join(",")
    );
    save_content("../data/gladius_death_or_glory/reels/fs_spin_multypliers.json".to_string(), result);
}



fn _convert_symbol_to_letter(symbol: char) -> char {
    match symbol {
        //empty cells
        '\\' => 'A',
        '^'  => 'B',
        '['  => 'C',
        ']'  => 'D',
        //collector empty
        ')' => 'E',
        //collector values any
        '*' => 'F',
        //coin values 5, 10, 25, 50
        'G'  => 'G',
        'H'  => 'H',
        'I'  => 'I',
        'J'  => 'J',
        //coin values 1, 2, 3, 4
        '='  => 'K',
        '>'  => 'L',
        '?'  => 'M',
        '@'  => 'N',
        //coin empty
        '(' => 'O',
        //bonus
        '3' => 'P',
        //coin values 100, 250, 500, 1000
        'Q'  => 'Q',
        'R'  => 'R',
        'S'  => 'S',
        'T'  => 'T',
        //multipliers 2, 3, 4, 5, 10, 20
        '+'  => 'U',
        ','  => 'V',
        '-'  => 'W',
        '.' => 'X',
        '/' => 'Y',
        '0' => 'Z',
        _ => symbol,
    }
}