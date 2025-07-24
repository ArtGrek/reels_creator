use serde_json::{Value/*, Map*/, json};
use std::io::{self};
use std::path::Path;
use std::fs;
//use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
use crate::netg::jocker_twenty::models::Category;

pub fn load_transactions (a_location: String) -> Vec<Value>{
    //print!("\x1B[2J\x1B[1;1H"); io::stdout().flush().unwrap();
    let transactions_file_path = loop {
        print!("Input transactions file path with name (optional): "); let _ = io::Write::flush(&mut io::stdout()); let mut transactions_file_path_input = String::new(); let _ = io::stdin().read_line(&mut transactions_file_path_input);
        if transactions_file_path_input.trim().is_empty() {break a_location;} else {
            let trimmed = transactions_file_path_input.trim().to_string();
            if Path::new(&transactions_file_path_input).is_dir() || Path::new(&transactions_file_path_input).is_file() {break trimmed;}
        }
    };
    let pb_main = ProgressBar::new((2) as u64);
    pb_main.set_prefix("Load transactions from ".to_owned() + &transactions_file_path + ": ");
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_transactions: Vec<Value> = Vec::new();
    if Path::new(&transactions_file_path).is_dir() {
        // определяем количество json-файлов для прогресс-бара
        let files: Vec<_> = std::fs::read_dir(&transactions_file_path).unwrap().filter_map(|e| {
            let p = e.unwrap().path();
            if p.extension().and_then(|s| s.to_str()) == Some("log") {Some(p)} else {None}
        }).collect();
        pb_main.set_length(files.len() as u64);
        for path in files {
            let content = std::fs::read_to_string(&path).unwrap();
            let wrapped = {
                let items = content.lines().map(str::trim).filter(|l| !l.is_empty()).collect::<Vec<_>>();
                format!("[{}]", items.join(","))
            };
            let data: Vec<Value> = serde_json::from_str(&wrapped).unwrap();
            // ИЗМЕНЕНИЕ: вместо общей фильтрации берём только initialMatrix
            for tx in data {
                if let Some(out_arr) = tx.get("out").and_then(|o| o.as_array()) {
                    for resp in out_arr {
                        // ищем именно GameStartBasicResponse (в поле "action")
                        if resp.get("action").and_then(|v| v.as_str()) == Some("GameStartBasicResponse") {
                            if let Some(mat) = resp.get("data").and_then(|d| d.get("initialMatrix")){
                                // формируем {"initialMatrix": mat}
                                l_transactions.push(json!({ "initialMatrix": mat.clone() }));
                            }
                        }
                    }
                }
            }
            pb_main.inc(1);
        }
    } else if Path::new(&transactions_file_path).is_file() {
        let file_content = fs::read_to_string(transactions_file_path).unwrap_or_default();
        pb_main.set_position(1);
        let data: Vec<Value> = serde_json::from_str(&file_content).unwrap_or_default();
        for tx in data {
            if let Some(out_arr) = tx.get("out").and_then(|o| o.as_array()) {
                for resp in out_arr {
                    if resp.get("action").and_then(|v| v.as_str()) == Some("GameStartBasicResponse") {
                        if let Some(mat) = resp.get("data").and_then(|d| d.get("initialMatrix")){l_transactions.push(json!({ "initialMatrix": mat.clone() }));}
                    }
                }
            }
        }
        pb_main.set_position(2);
    } else {println!("Does not exist or is not defined: {}", transactions_file_path);}
    pb_main.finish_with_message(" -> loaded ".to_owned() + &l_transactions.len().to_string() + " transactions");
    l_transactions
}

pub fn save_debug(a_categories: &Vec<Category>, a_location: &str, a_game_name: &str, a_categories_type: &str,) {
    // Базовая папка: "{a_location}/{a_game_name}/debug/{a_categories_type}/"
    let base_reels_dir = format!("{}/{}/debug/{}", a_location, a_game_name, a_categories_type);
    let _ = fs::create_dir_all(&base_reels_dir);

    let pb_main = ProgressBar::new(a_categories.len() as u64);
    pb_main.set_prefix("Saving ".to_owned() + &a_categories_type + " categories: ");
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    for (i, cat) in a_categories.iter().enumerate() {
        // Папка для этой категории: ".../debug/{a_categories_type}/category_{i}/"
        let cat_dir = format!("{}/category_{}", base_reels_dir, i);
        let _ = fs::create_dir_all(&cat_dir);
        //boards
        {
            // Вместо serde_json::to_string_pretty, формируем boards.json вручную
            let mut parts = Vec::new();
            // Helper для вывода Vec<i64> в одну строку
            let fmt_row = |row: &Vec<u8>| {
                let vals = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
                format!("[{}]", vals)
            };
            // 1) total: Vec<BoardsInstanse>
            /*let mut total_lines = Vec::new();
            for bs in &cat.boards.total {
                let inst = bs.instanses.iter().map(|row| format!("\t\t{}", fmt_row(row))).collect::<Vec<_>>().join(",\n");
                total_lines.push(format!("\t{{\n\t\t\"instanses\": [\n{}\n\t]\n\t}}", inst));
            }
            parts.push(format!("\t\"total\": [\n{}\n\t]", total_lines.join(",\n")));*/
            // 2) filtered: Vec<BoardsInstanse>
            /*let mut filt_lines = Vec::new();
            for bs in &cat.boards.filtered {
                let inst = bs.instanses.iter().map(|row| format!("\t\t{}", fmt_row(row))).collect::<Vec<_>>().join(",\n");
                filt_lines.push(format!("\t{{\n\t\t\"instanses\": [\n{}\n\t]\n\t}}", inst));
            }
            parts.push(format!("\t\"filtered\": [\n{}\n\t]", filt_lines.join(",\n")));*/
            // 3) unique: Vec<UniqueBoardsInstanse>
            let mut uniq_lines = Vec::new();
            for ub in &cat.boards.unique {
                let mut inst_lines = Vec::new();
                for board in &ub.instanses {
                    inst_lines.push(format!("\t\t{{\"count\":{},\"board\":{}}}",board.count,fmt_row(&board.board)));
                }
                let inst_block = inst_lines.join(",\n");
                uniq_lines.push(format!("\t{{\n\t\t\"count\":{},\n\t\t\"frequency_average\":{},\n\t\t\"instanses\":[\n{}\n\t\t]\n\t}}",ub.count, ub.frequency_average, inst_block));
            }
            parts.push(format!("\t\"unique\": [\n{}\n\t]", uniq_lines.join(",\n")));
            // 4) multiplied: Vec<BoardsInstanse>
            let mut mult_lines = Vec::new();
            for bs in &cat.boards.multiplied {
                let inst = bs.instanses.iter().map(|row| format!("\t\t{}", fmt_row(&row.board))).collect::<Vec<_>>().join(",\n");
                mult_lines.push(format!("\t{{\n\t\t\"instanses\": [\n{}\n\t]\n\t}}", inst));
            }
            parts.push(format!("\t\"multiplied\": [\n{}\n\t]", mult_lines.join(",\n")));
            // Собираем всё в один JSON-объект
            let json = format!("{{\n{}\n}}", parts.join(",\n"));
            fs::write(format!("{}/boards.json", &cat_dir), json).unwrap();
        }//
        //reels
        {
            // Вместо прямого to_string_pretty, строим JSON вручную
            let mut category_reels = Vec::new();
            for reel in &cat.reels {
                // Для каждого ReelInstanse собираем строку вида:
                // {"reel":[...],"remaining":[],"correct":true}
                let mut inst_lines = Vec::new();
                for inst in &reel.instanses {
                    let reel_json = serde_json::to_string(&inst.reel).unwrap();
                    let remaining_json = serde_json::to_string(&inst.remaining).unwrap();
                    let correct_json = inst.correct;
                    inst_lines.push(format!("{{\"reel\":{},\"remaining\":{},\"correct\":{}}}",reel_json, remaining_json, correct_json));
                }
                // Склеиваем все instanses в один блок
                let inst_block = inst_lines.join(",\n\t\t\t");
                // Добавляем объект для одного Reel
                category_reels.push(format!("\t{{\n\t\t\"instanses\": [\n\t\t\t{}\n\t\t]\n\t}}",inst_block));
            }
            let json = format!("[\n{}\n]", category_reels.join(",\n"));
            fs::write(format!("{}/reels.json", &cat_dir), json).unwrap();
        }//
        //spins
        /*{
            let _ = fs::write(format!("{}/spins.json", &cat_dir),serde_json::to_string_pretty(&cat.spins).unwrap(),);
        }//
        //bonus
        {
            // Для bonus — ручная «фиксация» ключей:
            let mut bonus_val = serde_json::to_value(&cat.bonus).unwrap();
            if let Value::Object(ref mut root) = bonus_val {
                // допустим, поле называется "values"
                if let Some(Value::Object(ref mut values_map)) = root.get_mut("values") {
                    // заберём старые пары
                    let old = std::mem::take(values_map);
                    // создаём новую Map<String,Value>, строковые ключи
                    let new: Map<String, Value> = old.into_iter().map(|(key_multi, v)| (key_multi.clone(), v)).collect();
                    *values_map = new;
                }
            }
            let bonus_json = serde_json::to_string_pretty(&bonus_val).unwrap();
            let _ = fs::write(format!("{}/bonus.json", &cat_dir), bonus_json);
        }// */
        pb_main.inc(1);
    }
    pb_main.finish_with_message(" -> categories saved");
}

pub fn save_reels (a_categories: Vec<Category>, _a_appearing_symbols: Vec<i64>, a_location: String, a_game_name: String, a_categories_type: &str, _width: usize, _height: usize,) {
    let format_reels = format!("{{\n\t\"reels\":{{\n{}\n\t}},\n\t\"bonus_appearance\":[\n{}\n\t],\n\t\"bonus_win\":{{{}}},\n\t\"bonus_init\":{{\n{}\n\t}},\n\t\"bonus_respin\":{{\n{}\n\t}}\n}}",
        //reels
        {
            let mut category_probability = 0.0;
            a_categories.iter().enumerate().map(|(_category_num, category)| {format!("\t\t\"{:.0}\":[\n{}\n\t\t]",
                {
                    category_probability += if a_categories.iter().map(|cat| {cat.count}).sum::<i64>() > 0 {
                        (category.count as f64) * 100.0 / (a_categories.iter().map(|cat| {cat.count}).sum::<i64>() as f64) * 100.0
                    } else {0.0};
                    category_probability
                },
                {
                    category.reels.iter().enumerate().map(|(_reel_num, reels)| {format!("\t\t\t[{}]",
                        reels.instanses.iter().filter(|instanse| {instanse.correct}).last().map(|r| {
                            let count = if r.reel.len() > 5 { r.reel.len() - 2 } else { r.reel.len() };
                            r.reel.iter().take(count).map(|x| format!("{}", x)).collect::<Vec<_>>().join(",")
                        }).unwrap_or_default()
                        )}).collect::<Vec<String>>().join(",\n")
                },
            )}).collect::<Vec<String>>().join(",\n")
        },
        format!(""),
        format!(""),
        format!(""),
        format!(""),
        //bonus_appearance
        /*
        {
            //let total_lenght: usize = a_category.iter().flat_map(|cat| cat.boards.total.iter().map(|b| b.instanses.len())).sum();
            let total_lenght: i64 = a_categories.iter().map(|cat| cat.spins.count).sum();
            let mut total_counts: Vec<(i64, Vec<i64>)> = a_appearing_symbols.iter().map(|&id| (id, vec![0; height])).collect();
            for category in &a_categories {
                for symbol in category.spins.symbols.iter().filter(|s| a_appearing_symbols.contains(&s.id)) {
                    if let Some((_, counts)) = total_counts.iter_mut().find(|(sym_id, _)| *sym_id == symbol.id)
                    {
                        for (_col_idx, col) in symbol.cols.iter().enumerate() {
                            for (row_idx, row) in col.rows.iter().enumerate() {
                                counts[row_idx] += row.count;
                            }
                        }
                    }
                }
            }
            let mut p = 0.0;
            let mut lines = Vec::new();
                for row_idx in 0..height {
                    let mut cell_entries = Vec::new();
                    for (sym_id, counts) in &total_counts {
                        let cnt = counts[row_idx];
                        p += cnt as f64 * 100.0 / total_lenght as f64 * 100.0 / 5.0;
                        let ratio = format!("{:.0}", p);
                        cell_entries.push(format!("\"{}\":{{\"pos\":[{}],\"id\":{}}}",
                            ratio, row_idx, sym_id
                        ));
                    }
                    lines.push(format!("\t\t{}", cell_entries.join(", ")));
                }
            format!("{}", lines.join(",\n"))
        },
        //bonus_win
        {
            let mut mech_totals: HashMap<String,(i64,i64)> = HashMap::new();
            for category in &a_categories {
                for mech in &category.bonus.by_mechanics {
                    let key = mech.id.iter().map(|n|n.to_string()).collect::<Vec<_>>().join("");
                    let entry = mech_totals.entry(key).or_insert((0,0));
                    entry.0 += mech.inits;
                    entry.1 += mech.appearances;
                }
            }
            let mut entries: Vec<(String,(i64,i64))> = mech_totals.into_iter().collect();
            entries.sort_by_key(|(id,_)| std::cmp::Reverse(id.parse::<i64>().unwrap()));
            entries.into_iter().map(|(id,(inits_sum,apps_sum))| {
                let pct = if apps_sum > 0 {inits_sum as f64 * 100.0 / apps_sum as f64 * 100.0} else {0.0}; 
                format!("\"{}\":{:.0}",id,pct)
            }).collect::<Vec<_>>().join(",")

        },
        //bonus_init
        {
            struct LenAgg {respins: i64,coins: HashMap<i64, i64>,coin_values: HashMap<i64, HashMap<String, i64>>,}
            // 1) Собираем и агрегируем по mech_id → length_idx
            let mut agg: HashMap<String, Vec<LenAgg>> = HashMap::new();
            for cat in &a_categories {
                for mech in &cat.bonus.by_mechanics {
                    let mech_id = mech.id.iter().map(|n| n.to_string()).collect::<String>();
                    let lens = agg.entry(mech_id.clone()).or_insert_with(|| {
                        mech.by_bonus_lenghts.iter().map(|_| LenAgg {respins: 0,coins: HashMap::new(),coin_values: HashMap::new(),}).collect()
                    });
                    for (i, by_len) in mech.by_bonus_lenghts.iter().enumerate() {
                        let la = &mut lens[i];
                        la.respins += by_len.respins;
                        // coin
                        for sym in &by_len.symbols {
                            if sym.id == 10 {
                                let sym_count: i64 = sym.count;
                                *la.coins.entry(sym.id).or_insert(0) += sym_count;
                                let vm = la.coin_values.entry(sym.id).or_insert_with(HashMap::new);
                                for (mv, cnt) in &sym.values {*vm.entry(mv.as_string()).or_insert(0) += *cnt;}
                            }
                        }
                    }
                }
            }
            // 2) Строим JSON-строку из агрегированных и отсортированных данных
            let mut mech_ids: Vec<String> = agg.keys().cloned().collect();
            mech_ids.sort_by_key(|id| id.parse::<i64>().unwrap_or(0));
            let mut mech_blocks = Vec::new();
            for mech_id in mech_ids {
                let lengths = &agg[&mech_id];
                let mut len_blocks = Vec::new();
                for (len_idx, la) in lengths.iter().enumerate() {
                    if (5 + mech_id.len() - 2) == len_idx {
                        let mut sids: Vec<i64> = la.coins.keys().cloned().collect();
                        sids.sort();
                        let mut val_entries = Vec::new();
                        for sid in sids {
                            let cnt = la.coins[&sid];
                            let mut cum_val_p = 0.0;
                            if let Some(vm) = la.coin_values.get(&sid) {
                                // сортируем по значению ключа (числовые по возрастанию, затем строки)
                                let mut vals: Vec<(&String, &i64)> = vm.iter().collect();
                                vals.sort_by(|(mv1, _), (mv2, _)| {
                                    match (mv1.parse::<f64>(), mv2.parse::<f64>()) {
                                        (Ok(n1), Ok(n2)) => n1.partial_cmp(&n2).unwrap_or(std::cmp::Ordering::Equal),
                                        (Ok(_), Err(_))  => std::cmp::Ordering::Less,
                                        (Err(_), Ok(_))  => std::cmp::Ordering::Greater,
                                        (Err(_), Err(_)) => {
                                            // специальный порядок для трёх строк
                                            let special = ["mini", "minor", "major"];
                                            let pos1 = special.iter().position(|&s| s == *mv1);
                                            let pos2 = special.iter().position(|&s| s == *mv2);
                                            match (pos1, pos2) {
                                                (Some(i1), Some(i2)) => i1.cmp(&i2),
                                                (Some(_), None)      => std::cmp::Ordering::Less,
                                                (None, Some(_))      => std::cmp::Ordering::Greater,
                                                (None, None)         => mv1.cmp(mv2),
                                            }
                                        },
                                    }
                                });
                                for (mv, &vc) in vals {
                                    let p_val = if cnt > 0 {vc as f64 * 100.0 / cnt as f64 * 100.0} else { 0.0 };
                                    cum_val_p += p_val;
                                    if p_val > 0.0 {val_entries.push(format!("\"{:.0}\":\"{}\"", cum_val_p, mv));} 
                                    else {val_entries.push(format!("\"{:.0}\":\"{}\"", 0.0, mv));}
                                }
                            }
                        }

                        len_blocks.push(format!("{}", val_entries.join(",")));
                    }
                }
                mech_blocks.push(format!("\t\t\"{}\":{{{}}}", mech_id, len_blocks.join(",\n")));
            }
            // 3) Финальный JSON-блок
            format!("{}", mech_blocks.join(",\n"))
        },
        //bonus_respin
        {
            struct LenAgg {respins: i64,coins: HashMap<i64, i64>,coin_values: HashMap<i64, HashMap<String, i64>>,symbols: HashMap<i64, i64>,symbol_values: HashMap<i64, HashMap<String, i64>>,mystery: HashMap<i64, i64>,mystery_values: HashMap<i64, HashMap<String, i64>>,}
            // 1) Собираем и агрегируем по mech_id → length_idx
            let mut agg: HashMap<String, Vec<LenAgg>> = HashMap::new();
            for cat in &a_categories {
                for mech in &cat.bonus.by_mechanics {
                    let mech_id = mech.id.iter().map(|n| n.to_string()).collect::<String>();
                    let lens = agg.entry(mech_id.clone()).or_insert_with(|| {
                        mech.by_bonus_lenghts.iter().map(|_| LenAgg {respins: 0,coins: HashMap::new(),coin_values: HashMap::new(),symbols: HashMap::new(),symbol_values: HashMap::new(),mystery: HashMap::new(),mystery_values: HashMap::new(),}).collect()
                    });
                    for (i, by_len) in mech.by_bonus_lenghts.iter().enumerate() {
                        let la = &mut lens[i];
                        la.respins += by_len.respins;
                        // coin
                        for sym in &by_len.symbols {
                            if sym.id == 10 {
                                let sym_count: i64 = sym.count;
                                *la.coins.entry(sym.id).or_insert(0) += sym_count;
                                let vm = la.coin_values.entry(sym.id).or_insert_with(HashMap::new);
                                for (mv, cnt) in &sym.values {*vm.entry(mv.as_string()).or_insert(0) += *cnt;}
                            }
                        }
                        // symbols
                        for sym in &by_len.symbols {
                            if sym.id == 11 || sym.id == 12 || sym.id == 13 {
                                let sym_count: i64 = sym.count;
                                *la.symbols.entry(sym.id).or_insert(0) += sym_count;
                                let vm = la.symbol_values.entry(sym.id).or_insert_with(HashMap::new);
                                for (mv, cnt) in &sym.values {*vm.entry(mv.as_string()).or_insert(0) += *cnt;}
                            }
                        }
                        // mystery
                        for sym in &by_len.mystery_symbols {
                            let myst_count: i64 = sym.count;
                            *la.mystery.entry(sym.id).or_insert(0) += myst_count;
                            let vm = la.mystery_values.entry(sym.id).or_insert_with(HashMap::new);
                            for (mv, cnt) in &sym.values {*vm.entry(mv.as_string()).or_insert(0) += *cnt;}
                        }
                    }
                }
            }
            // 2) Строим JSON-строку из агрегированных и отсортированных данных
            let mut mech_ids: Vec<String> = agg.keys().cloned().collect();
            mech_ids.sort_by_key(|id| id.parse::<i64>().unwrap_or(0));
            let mut mech_blocks = Vec::new();
            for mech_id in mech_ids {
                let lengths = &agg[&mech_id];
                let mut len_blocks = Vec::new();
                for (len_idx, la) in lengths.iter().enumerate() {
                    if (5 + mech_id.len() - 1) <= len_idx {

                        // coins 
                        let mut cum_sym_p = 0.0;
                        let mut sym_entries = Vec::new();
                        let mut sids: Vec<i64> = la.coins.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.coins[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 * 100.0 / 15.0} else { 0.0 };
                            cum_sym_p += p_sym;
                            let mut cum_val_p = 0.0;
                            let mut val_entries = Vec::new();
                            if let Some(vm) = la.coin_values.get(&sid) {
                                // сортируем по значению ключа (числовые по возрастанию, затем строки)
                                let mut vals: Vec<(&String, &i64)> = vm.iter().collect();
                                vals.sort_by(|(mv1, _), (mv2, _)| {
                                    match (mv1.parse::<f64>(), mv2.parse::<f64>()) {
                                        (Ok(n1), Ok(n2)) => n1.partial_cmp(&n2).unwrap_or(std::cmp::Ordering::Equal),
                                        (Ok(_), Err(_))  => std::cmp::Ordering::Less,
                                        (Err(_), Ok(_))  => std::cmp::Ordering::Greater,
                                        (Err(_), Err(_)) => {
                                            // специальный порядок для трёх строк
                                            let special = ["mini", "minor", "major"];
                                            let pos1 = special.iter().position(|&s| s == *mv1);
                                            let pos2 = special.iter().position(|&s| s == *mv2);
                                            match (pos1, pos2) {
                                                (Some(i1), Some(i2)) => i1.cmp(&i2),
                                                (Some(_), None)      => std::cmp::Ordering::Less,
                                                (None, Some(_))      => std::cmp::Ordering::Greater,
                                                (None, None)         => mv1.cmp(mv2),
                                            }
                                        },
                                    }
                                });
                                for (mv, &vc) in vals {
                                    let p_val = if cnt > 0 {vc as f64 * 100.0 / cnt as f64 * 100.0} else { 0.0 };
                                    
                                    cum_val_p += p_val;
                                    if p_val > 0.0 {val_entries.push(format!("\"{:.0}\":\"{}\"", cum_val_p, mv));} 
                                    else {val_entries.push(format!("\"{:.0}\":\"{}\"", 0.0, mv));}
                                }
                            }
                            if p_sym > 0.0 {sym_entries.push(format!("\t\t\t\t\t\"{:.0}\":{{{}}}",cum_sym_p, val_entries.join(",")));} 
                            else {sym_entries.push(format!("\t\t\t\t\t\"{:.0}\":{{{}}}",0.0, val_entries.join(",")));}
                        }
                        let coins_block = format!("\t\t\t\t\"coins\":{{\n{}\n\t\t\t\t}}", sym_entries.join(",\n"));

                        // symbols (без изменений)
                        let mut cum_sym_p = 0.0;
                        let mut sym_entries = Vec::new();
                        let mut sids: Vec<i64> = la.symbols.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.symbols[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 * 100.0} else { 0.0 };
                            cum_sym_p += p_sym;
                            let mut cum_val_p = 0.0;
                            let mut val_entries = Vec::new();
                            if let Some(vm) = la.symbol_values.get(&sid) {
                                // сортируем по значению ключа (числовые по возрастанию, затем строки)
                                let mut vals: Vec<(&String, &i64)> = vm.iter().collect();
                                vals.sort_by(|(mv1, _), (mv2, _)| {
                                    match (mv1.parse::<f64>(), mv2.parse::<f64>()) {
                                        (Ok(n1), Ok(n2)) => n1.partial_cmp(&n2).unwrap_or(std::cmp::Ordering::Equal),
                                        (Ok(_), Err(_))  => std::cmp::Ordering::Less,
                                        (Err(_), Ok(_))  => std::cmp::Ordering::Greater,
                                        (Err(_), Err(_)) => {
                                            // специальный порядок для трёх строк
                                            let special = ["mini", "minor", "major"];
                                            let pos1 = special.iter().position(|&s| s == *mv1);
                                            let pos2 = special.iter().position(|&s| s == *mv2);
                                            match (pos1, pos2) {
                                                (Some(i1), Some(i2)) => i1.cmp(&i2),
                                                (Some(_), None)      => std::cmp::Ordering::Less,
                                                (None, Some(_))      => std::cmp::Ordering::Greater,
                                                (None, None)         => mv1.cmp(mv2),
                                            }
                                        },
                                    }
                                });
                                for (mv, &vc) in vals {
                                    let p_val = if cnt > 0 {vc as f64 * 100.0 / cnt as f64 * 100.0} else { 0.0 };
                                    cum_val_p += p_val;
                                    if p_val > 0.0 {val_entries.push(format!("\"{:.0}\":\"{}\"", cum_val_p, mv));} 
                                    else {val_entries.push(format!("\"{:.0}\":\"{}\"", 0.0, mv));}
                                }
                            }
                            if p_sym > 0.0 {sym_entries.push(format!("\t\t\t\t\t\"{:.0}\":{{\"id\":\"{}\",\"values\":{{{}}}}}",cum_sym_p, sid, val_entries.join(",")));} 
                            else {sym_entries.push(format!("\t\t\t\t\t\"{:.0}\":{{\"id\":\"{}\",\"values\":{{{}}}}}",0.0, sid, val_entries.join(",")));}
                        }
                        let symbols_block = format!("\t\t\t\t\"symbols\":{{\n{}\n\t\t\t\t}}", sym_entries.join(",\n"));

                        // mystery – вероятность появления mystery-символа и его замены
                        let total_myst: i64 = la.mystery.values().sum();
                        let block_prob = if la.respins > 0 {total_myst as f64 * 100.0 / la.respins as f64 * 100.0} else { 0.0 };
                        let mut cum_sym_p = 0.0;
                        let mut myst_entries = Vec::new();
                        let mut mids: Vec<i64> = la.mystery.keys().cloned().collect();
                        mids.sort();
                        for sid in mids {
                            let sym_count = la.mystery[&sid];
                            let sym_prob = if total_myst > 0 {sym_count as f64 * 100.0 / total_myst as f64 * 100.0} else { 0.0 };
                            cum_sym_p += sym_prob;
                            let mut cum_val_p = 0.0;
                            let mut val_entries = Vec::new();
                            if let Some(vm) = la.mystery_values.get(&sid) {
                                // сортировка по значению ключа
                                let mut vals: Vec<(&String, &i64)> = vm.iter().collect();
                                vals.sort_by(|(mv1, _), (mv2, _)| {
                                    match (mv1.parse::<f64>(), mv2.parse::<f64>()) {
                                        (Ok(n1), Ok(n2)) => n1.partial_cmp(&n2).unwrap_or(std::cmp::Ordering::Equal),
                                        (Ok(_), Err(_))  => std::cmp::Ordering::Less,
                                        (Err(_), Ok(_))  => std::cmp::Ordering::Greater,
                                        (Err(_), Err(_)) => {
                                            // специальный порядок для трёх строк
                                            let special = ["mini", "minor", "major"];
                                            let pos1 = special.iter().position(|&s| s == *mv1);
                                            let pos2 = special.iter().position(|&s| s == *mv2);
                                            match (pos1, pos2) {
                                                (Some(i1), Some(i2)) => i1.cmp(&i2),
                                                (Some(_), None)      => std::cmp::Ordering::Less,
                                                (None, Some(_))      => std::cmp::Ordering::Greater,
                                                (None, None)         => mv1.cmp(mv2),
                                            }
                                        },
                                    }
                                });
                                for (mv, &vc) in vals {
                                    let val_prob = if sym_count > 0 {vc as f64 * 100.0 / sym_count as f64 * 100.0} else { 0.0 };
                                    cum_val_p += val_prob;
                                    if val_prob > 0.0 {val_entries.push(format!("\"{:.0}\":\"{}\"", cum_val_p, mv));} 
                                    else {val_entries.push(format!("\"{:.0}\":\"{}\"", 0.0, mv));}
                                }
                            }
                            if sym_prob > 0.0 {myst_entries.push(format!("\t\t\t\t\t\"{:.0}\":{{\"id\":\"{}\",\"values\":{{{}}}}}",cum_sym_p, sid, val_entries.join(",")));}
                            else {myst_entries.push(format!("\t\t\t\t\t\"{:.0}\":{{\"id\":\"{}\",\"values\":{{{}}}}}",0.0, sid, val_entries.join(",")));}
                        }
                        let mystery_block = format!("\t\t\t\t\"mystery\":{{\"{:.0}\":{{\n{}\n\t\t\t\t}}}}",block_prob,myst_entries.join(",\n"));

                        len_blocks.push(format!("\t\t\t\"{}\":{{\n{},\n{},\n{}\n\t\t\t}}", len_idx, coins_block, symbols_block, mystery_block));
                    }
                }
                mech_blocks.push(format!("\t\t\"{}\":{{\n{}\n\t\t}}", mech_id, len_blocks.join(",\n")));
            }
            // 3) Финальный JSON-блок
            format!("{}", mech_blocks.join(",\n"))
        },*/
    
    );
    if let Some(parent) = Path::new(&(a_location.to_owned() + &a_game_name.clone() + "/reels/reels" + a_categories_type + ".json")).parent() {let _ = fs::create_dir_all(parent);}
    fs::write(a_location.to_owned() + &a_game_name.clone() + "/reels/reels" + a_categories_type + ".json", format_reels).unwrap();
    
}
