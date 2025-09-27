use serde_json::Value;
use crate::storage::{load_transactions, save_content, };

pub fn extract_base_reels() {
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
    let reels = combos.iter().map(|combo| {
        let mut posibility = 0;
        let col = (0..(1usize << 5)).map(|m|  {
            let pattern = (0..5).map(|b| {if (m >> b) & 1 == 1 { "\"@\"" } else { "\"O\"" }}).collect::<Vec<&str>>().join(",");
            if combo[m] > 0 {
                posibility += combo[m];
                format!("\t\t\t\"{}\": [{}]", posibility, pattern)
            } else {format!("\t\t\t\"{}\": [{}]", combo[m], pattern)}
        }).collect::<Vec<String>>().join(",\n");
        format!("\t\t{{\n{}\n\t\t}}", col)
    }).collect::<Vec<_>>().join(",\n");
    save_content("../data/gladius_death_or_glory/reels/base_reels.json".to_string(), format!("[\n\t[\n{}\n\t]\n]", reels));
}


pub fn extract_base_coin_values() {
    let transactions: Vec<Value> =load_transactions("../data/gladius_death_or_glory/transactions/spin/".to_string());
    let mut grids_count = 0usize;

    let mut counts = [0usize; 12]; 
    let equiv = ['K','L','M','N','G','H','I','J','Q','R','S','T'];

    for tx in &transactions {
        if let Some(events) = tx["out"]["round"]["events"].as_array() {
            for ev in events {
                if let Some(etn) = ev["etn"].as_str() {
                    if etn.starts_with("coin_reveal_") {
                        if let Some(grid) = ev["c"]["grid"].as_str() {
                            grids_count += 1;
                            let chars: Vec<char> = grid.chars().skip(2).collect();

                            for ch in chars {
                                match ch {
                                    '=' => counts[0] += 1, // K
                                    '>' => counts[1] += 1, // L
                                    '?' => counts[2] += 1, // M
                                    '@' => counts[3] += 1, // N
                                    'G' => counts[4] += 1, // G
                                    'H' => counts[5] += 1, // H
                                    'I' => counts[6] += 1, // I
                                    'J' => counts[7] += 1, // J
                                    'Q' => counts[8] += 1, // Q
                                    'R' => counts[9] += 1, // R
                                    'S' => counts[10] += 1, // S
                                    'T' => counts[11] += 1, // T
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
    let coins_count: usize = counts.iter().sum();
    println!("total coins count: {}", coins_count);
    
    let mut posibility = 0;
    let coin_values = (0..equiv.len()).map(|i| {
        if counts[i] > 0 {
            posibility += counts[i];
            format!("\t\"{}\": \"{}\"", posibility, equiv[i])
        } else {format!("\t\"{}\": \"{}\"", counts[i], equiv[i])}
    }).collect::<Vec<_>>().join(",\n");
    save_content("../data/gladius_death_or_glory/reels/base_coin_values.json".to_string(), format!("{{\n{}\n}}", coin_values));
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