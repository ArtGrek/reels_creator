use serde_json::{Value, Map};
use std::path::Path;
use std::fs;
use std::collections::BTreeMap;
use std::collections::HashMap;
use crate::bng::china_festival::models::Categories;

pub fn save_fugaso (a_categories: Categories, a_appearing_symbols: Vec<i64>, a_location: String, a_game_name: String) {
    let format_reels = format!("{{\n\t\"stopFactor\": {},\n\t\"betCounters\": {},\n\t\"distBuy\": {},\n\t\"distRespin\": {},\n\t\"distCoin\": {},\n\t\"distBaseCategory\": {},\n\t\"distSymbol\": [{}, 10000],\n\t\"distMystery\": {{\n{}\n\t}},\n\t\"distBombs\": [{}],\n\t\"distBonus\": {},\n\t\"distNewCoin\": [{}],\n\t\"reels\": [\n{}\n\t],\n\t\"reelsBonus\": {{\n{}\n\t}},\n\t\"lines\": {},\n\t\"wins\": {}\n}}",
        //stopFactor
        10000,
        //betCounters
        {
            let p1 = a_categories.settings.get("buy_bonus_price_1").and_then(Value::as_i64).unwrap();
            let p2 = a_categories.settings.get("buy_bonus_price_2").and_then(Value::as_i64).unwrap();
            let json = format!("[\n\t\t{},\n\t\t{},\n\t\t{}\n\t]", 1, p1, p2);
            json
        },
        //distBuy
        {
            // 1) Собираем суммарные inits по механикам для двух групп
            let mut dist1 = BTreeMap::new();
            for cat in &a_categories.buy_category[0] {
                for mech in &cat.bonus.by_mechanics {
                    let name = mechanics_name(mech.id.clone());
                    *dist1.entry(name).or_insert(0) += mech.inits;
                }
            }
            let mut dist2 = BTreeMap::new();
            for cat in &a_categories.buy_category[1] {
                for mech in &cat.bonus.by_mechanics {
                    let name = mechanics_name(mech.id.clone());
                    *dist2.entry(name).or_insert(0) += mech.inits;
                }
            }
            // общий базовый inits для %
            let total1: i64 = a_categories.buy_category[0].iter().map(|cat| cat.bonus.inits).sum();
            let total2: i64 = a_categories.buy_category[1].iter().map(|cat| cat.bonus.inits).sum();
        
            // 2) Формируем строки для первого объекта
            let mut lines1 = Vec::new();
            let mut p = 0.0;
            for (mech, &sum) in &dist1 {
                let pct = if total1 != 0 { sum as f64 * 100.0 / total1 as f64 * 100.0 } else { 0.0 };
                p += pct;
                lines1.push(format!("\t\t\"{:.0}\": \"{}\"", p, mech));
            }
        
            // 3) То же для второго
            let mut lines2 = Vec::new();
            let mut p = 0.0;
            for (mech, &sum) in &dist2 {
                let pct = if total2 != 0 { sum as f64 * 100.0 / total2 as f64 * 100.0 } else { 0.0 };
                p += pct;
                lines2.push(format!("\t\t\"{:.0}\": \"{}\"", p, mech));
            }
        
            // 4) Собираем финальную строку
            let mut s = String::new();
            s.push_str("[{\n");
            s.push_str(&lines1.join(",\n"));
            s.push_str("\n    }, {\n");
            s.push_str(&lines2.join(",\n"));
            s.push_str("\n    }]");
            s
        },
        //distRespin
        {
            // 1) Собирать суммарные inits и appearances по mechanic.id
            let mut sum_inits       = BTreeMap::<String, i64>::new();
            let mut sum_appearances = BTreeMap::<String, i64>::new();
            for cat in &a_categories.category {
                for mech in &cat.bonus.by_mechanics {
                    *sum_inits
                        .entry(mechanics_name(mech.id.clone()))
                        .or_insert(0) += mech.inits;
                    *sum_appearances
                        .entry(mechanics_name(mech.id.clone()))
                        .or_insert(0) += mech.appearances;
                }
            }
        
            // 2) Формируем строки вида `"mechanic_id": [sum_inits, sum_appearances]`
            let mut lines = Vec::new();
            for (id, &inits) in &sum_inits {
                let appears = sum_appearances.get(id).copied().unwrap_or(0);
                lines.push(format!("\t\t\"{}\": [{:.0}, 10000]", id, inits as f64 * 100.0 / appears as f64 * 100.0));
            }
        
            // 3) Собираем итоговый JSON с переносами и отступами
            let mut s = String::new();
            s.push_str("{\n");
            s.push_str(&lines.join(",\n"));
            s.push_str("\n\t}");
            s
        },
        //distCoin
        {
            // 1) Собираем dist как раньше
            let mut dist = BTreeMap::new();
            for cat in &a_categories.category {
                if let Some(idx) = cat.bonus.symbols.iter().position(|s| s.id == 14) {
                    let sym = &cat.bonus.symbols[idx];
                    for (key, &val) in &sym.values {
                        *dist.entry(key.as_string()).or_insert(0) += val;
                    }
                }
            }
        
            // 2) Считаем total и bet_factor
            let total: i64 = a_categories
                .category
                .iter()
                .filter_map(|cat| {
                    cat.bonus
                        .symbols
                        .iter()
                        .find(|s| s.id == 14)
                        .map(|s| s.count)
                })
                .sum();
        
            let bet_factor: i64 = a_categories
                .settings
                .get("bet_factor")
                .and_then(Value::as_array)
                .and_then(|arr| arr.get(0))
                .and_then(Value::as_i64)
                .unwrap_or(1);
        
            let empty_jackpots: Map<String, Value> = Map::new();
            let jackpots = a_categories
                .settings
                .get("jackpots")
                .and_then(Value::as_object)
                .unwrap_or(&empty_jackpots);
        
            // 3) Собираем вектор пар (pct, new_k)
            let mut pairs: Vec<(i64, i64)> = dist.iter().map(|(k, &v)| {
                // вычисляем new_k
                let new_k_f = if let Ok(n) = k.parse::<f64>() {
                    n * bet_factor as f64
                } else {
                    jackpots.get(k).and_then(Value::as_f64).unwrap_or(0.0) * bet_factor as f64
                };
                let new_k = new_k_f.round() as i64;
        
                // считаем pct
                let pct_f = (v as f64 * 100.0 / total as f64) * 100.0;
                let pct = pct_f.round() as i64;
        
                (pct, new_k)
            }).collect();
        
            // 4) Сортируем по new_k (второму элементу кортежа)
            pairs.sort_by_key(|&(_pct, new_k)| new_k);
        
            // 5) Формируем строки с отступом и собираем JSON
            let mut p = 0;
            let lines: Vec<String> = pairs.into_iter().map(|(pct, new_k)| {
                p += pct;
                format!("\t\t\"{}\": {}", p, new_k)
            }).collect();
        
            // 6) Финальная строка
            format!("{{\n{}\n\t}}", lines.join(",\n"))
        },
        //distBaseCategory
        {
            let mut p = 0.0;
            let map: Vec<String> = a_categories.category.iter().enumerate().map(|(cat_num, cat)| {
                p += (cat.spins.count as f64)*100.0/(a_categories.category.iter().map(|acat| {acat.spins.count}).sum::<i64>() as f64)*100.0;
                format!("\t\t\"{:.0}\": {}", p, cat_num)
            }).collect();
            format!("{{\n{}\n\t}}", map.join(",\n"))
        },
        //distSymbol
        {
            /*// 1) Выбираем все sym.count по фильтру и считаем их сумму и количество
            let (sum_sym_count, sym_count_len) = a_categories.category.iter().flat_map(|cat| {
                    cat.spins.symbols.iter().filter(|sym| a_appearing_symbols.contains(&sym.id)).map(|sym| sym.count)
            }).fold((0i64, 0usize), |(sum, cnt), v| (sum + v, cnt + 1));
            // вычисляем среднее арифметическое
            let avg_sym_count = if sym_count_len > 0 {sum_sym_count as f64 / sym_count_len as f64} else {0.0};
            // 2) Считаем total_spins_count
            let total_spins_count: i64 = a_categories.category.iter().map(|cat| cat.spins.count).sum();
            // 3) Формируем строку ответа
            let result = if total_spins_count != 0 {
                // если нужно в процентах:
                // avg_sym_count * 100.0 / total_spins_count as f64 * 100.0
                format!("{:.0}", avg_sym_count * 100.0 / total_spins_count as f64 * 100.0 * 3.0 * 3.0 / 5.0)} else {"0".to_string()};
            result*/

            //let total_lenght: usize = a_categories.category.iter().flat_map(|cat| cat.boards.total.iter().map(|b| b.instanses.len())).sum();
            let total_lenght: i64 = a_categories.category.iter().map(|cat| cat.spins.count).sum();
            let mut total_counts: Vec<(i64, Vec<i64>)> = a_appearing_symbols.iter().map(|&id| (id, vec![0; 3])).collect();
            for category in &a_categories.category {
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
                for row_idx in 0..3 {
                    for (_sym_id, counts) in &total_counts {
                        let cnt = counts[row_idx];
                        p += cnt as f64 * 100.0 / total_lenght as f64 * 100.0 / 5.0;
                    }
                }
            if total_lenght != 0 {format!("{:.0}", p)} else {"0".to_string()}
        },
        //distMystery
        {
            let mut mystery_count: i64 = 0;
            if let Some(idx) = a_categories.category[0].bonus.symbols.iter().position(|s| s.id == 15) {
                mystery_count = a_categories.category[0].bonus.symbols[idx].count;
            }
            let mut p = 0.0;
            a_categories.category[0].bonus.mystery_symbols.iter().map(|sym| {
                p += sym.count as f64 * 100.0 / mystery_count as f64 * 100.0;
                format!("\t\t\"{:.0}\": \"{}\"", p, num_str_to_letter(&sym.id.to_string()).unwrap())
            }).collect::<Vec<String>>().join(",\n")
        },
        //distBombs
        "[{\"100\":1},{\"25\":1,\"100\":0},{\"20\":1,\"100\":0},{\"15\":1,\"100\":0},{\"10\":1,\"100\":0},{\"5\":1,\"100\":0}]",
        //distBonus
        {
            // 1) Собираем агрегированные данные:
            //    механика (Vec<i64>) → (total_reappearances, map combo_key → total_value)
            let mut agg: BTreeMap<Vec<i64>, (i64, BTreeMap<Vec<i64>, i64>)> = BTreeMap::new();
            for cat in a_categories.category.clone() {
                for mech in &cat.bonus.by_mechanics {
                    let entry = agg
                        .entry(mech.id.clone())
                        .or_insert((0, BTreeMap::new()));
                    // суммируем reappearances
                    entry.0 += mech.reappearances;
                    // и для каждого combo_symbol: суммируем по ключу
                    for (combo_key, &val) in &mech.combo_symbols {
                        *entry.1.entry(combo_key.clone()).or_insert(0) += val;
                    }
                }
            }
            // 2) Формируем строки вида:
            //    "mechanic_id": { "combo_value/total": "combo_key", … }
            let mut lines = Vec::new();
            for (mech_id, (total_reap, combo_map)) in agg {
                // Преобразуем Vec<i64> в понятный ключ, например "Add", "Twice", …
                let mech_id_str = mechanics_name(mech_id);
                let mut inner = Vec::new();
                let mut p = 0.0;
                for (combo_key, combo_val) in combo_map {
                    // пропускаем нулевые
                    if combo_val == 0 {
                        continue;
                    }
                    if total_reap != 0 {
                        // накапливаем процент
                        p += combo_val as f64 * 100.0 / total_reap as f64 * 100.0;
                        let ratio = format!("{:.0}", p);
                        // строим строку из массива ключей, например [1,2] → "AB"
                        let combo_key_str = combo_key
                            .iter()
                            .filter_map(|key| num_str_to_letter(&key.to_string()))
                            .collect::<Vec<_>>()
                            .join("");
                        inner.push(format!("\t\t\t\"{}\":\"{}\"", ratio, combo_key_str));
                    }
                }
                // если для этой механики что-то накопилось — добавляем в вывод
                if !inner.is_empty() {
                    let inner_block = inner.join(",\n");
                    lines.push(format!("\t\t\"{}\":{{\n{}\n\t\t}}", mech_id_str, inner_block));
                }
            }
            // 3) Собираем итоговый JSON-объект
            format!("{{\n{}\n\t}}", lines.join(",\n"))
        },
        //distNewCoin
        {
            struct LenAgg {respins: i64,coins: HashMap<i64, i64>,coin_values: HashMap<i64, HashMap<String, i64>>,symbols: HashMap<i64, i64>,symbol_values: HashMap<i64, HashMap<String, i64>>,mystery: HashMap<i64, i64>,mystery_values: HashMap<i64, HashMap<String, i64>>,}
            let mut total_blocks = Vec::new();

            // 1) Собираем и агрегируем по mech_id → length_idx
            let mut agg: HashMap<String, Vec<LenAgg>> = HashMap::new();
            for cat in &a_categories.category {
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
                            if sym.id == 14 {
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
            let mut mech_blocks1 = Vec::new();
            for mech_id in mech_ids {
                let lengths = &agg[&mech_id];
                let mut len_blocks = Vec::new();
                for (len_idx, la) in lengths.iter().enumerate() {
                    if (5 + mech_id.len() - 1) <= len_idx {

                        // coins 
                        let mut cum_sym_p = 0.0;
                        let mut sids: Vec<i64> = la.coins.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.coins[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                            cum_sym_p += p_sym;
                        }
                        let coins_block = format!("{:.7}",cum_sym_p);

                        // symbols (без изменений)
                        let mut cum_sym_p = 0.0;
                        let mut sids: Vec<i64> = la.symbols.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.symbols[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                            cum_sym_p += p_sym;
                        }
                        let symbols_block = format!("{:.7}",cum_sym_p);

                        // mystery – вероятность появления mystery-символа и его замены
                        let total_myst: i64 = la.mystery.values().sum();
                        let block_prob = if la.respins > 0 {total_myst as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                        let mystery_block = format!("{:.7}",block_prob);

                        len_blocks.push(format!("\t\t\t[{},{},{}]", coins_block, symbols_block, mystery_block));
                    }
                }
                mech_blocks1.push(format!("\t\t\"{}\":[\n{}\n\t\t]", mechanics_str_name(&mech_id), len_blocks.join(",\n")));
            }
            // 3) Финальный JSON-блок
            total_blocks.push(format!("{{{}}}", mech_blocks1.join(",\n")));

            // 1) Собираем и агрегируем по mech_id → length_idx
            let mut agg: HashMap<String, Vec<LenAgg>> = HashMap::new();
            for cat in &a_categories.buy_category[0] {
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
                            if sym.id == 14 {
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
            let mut mech_blocks2 = Vec::new();
            for mech_id in mech_ids {
                let lengths = &agg[&mech_id];
                let mut len_blocks = Vec::new();
                for (len_idx, la) in lengths.iter().enumerate() {
                    if (5 + mech_id.len() - 1) <= len_idx {

                        // coins 
                        let mut cum_sym_p = 0.0;
                        let mut sids: Vec<i64> = la.coins.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.coins[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                            cum_sym_p += p_sym;
                        }
                        let coins_block = format!("{:.7}",cum_sym_p);

                        // symbols (без изменений)
                        let mut cum_sym_p = 0.0;
                        let mut sids: Vec<i64> = la.symbols.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.symbols[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                            cum_sym_p += p_sym;
                        }
                        let symbols_block = format!("{:.7}",cum_sym_p);

                        // mystery – вероятность появления mystery-символа и его замены
                        let total_myst: i64 = la.mystery.values().sum();
                        let block_prob = if la.respins > 0 {total_myst as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                        let mystery_block = format!("{:.7}",block_prob);

                        len_blocks.push(format!("\t\t\t[{},{},{}]", coins_block, symbols_block, mystery_block));
                    }
                }
                mech_blocks2.push(format!("\t\t\"{}\":[\n{}\n\t\t]", mechanics_str_name(&mech_id), len_blocks.join(",\n")));
            }
            // 3) Финальный JSON-блок
            total_blocks.push(format!("{{{}}}", mech_blocks2.join(",\n")));

            // 1) Собираем и агрегируем по mech_id → length_idx
            let mut agg: HashMap<String, Vec<LenAgg>> = HashMap::new();
            for cat in &a_categories.buy_category[1] {
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
                            if sym.id == 14 {
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
            let mut mech_blocks3 = Vec::new();
            for mech_id in mech_ids {
                let lengths = &agg[&mech_id];
                let mut len_blocks = Vec::new();
                for (len_idx, la) in lengths.iter().enumerate() {
                    if (5 + mech_id.len() - 1) <= len_idx {

                        // coins 
                        let mut cum_sym_p = 0.0;
                        let mut sids: Vec<i64> = la.coins.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.coins[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                            cum_sym_p += p_sym;
                        }
                        let coins_block = format!("{:.7}",cum_sym_p);

                        // symbols (без изменений)
                        let mut cum_sym_p = 0.0;
                        let mut sids: Vec<i64> = la.symbols.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.symbols[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                            cum_sym_p += p_sym;
                        }
                        let symbols_block = format!("{:.7}",cum_sym_p);

                        // mystery – вероятность появления mystery-символа и его замены
                        let total_myst: i64 = la.mystery.values().sum();
                        let block_prob = if la.respins > 0 {total_myst as f64 * 100.0 / la.respins as f64 / 15.0 / 100.0} else { 0.0 };
                        let mystery_block = format!("{:.7}",block_prob);

                        len_blocks.push(format!("\t\t\t[{},{},{}]", coins_block, symbols_block, mystery_block));
                    }
                }
                mech_blocks3.push(format!("\t\t\"{}\":[\n{}\n\t\t]", mechanics_str_name(&mech_id), len_blocks.join(",\n")));
            }
            // 3) Финальный JSON-блок
            total_blocks.push(format!("{{{}}}", mech_blocks3.join(",\n")));
            
            format!("\n{}\n", total_blocks.join(",\n"))
        },
        //reels
        {
            a_categories.category.iter().map(|category| {
                format!("\t\t[\n{}\n\t\t]", 
                    category.reels.iter().map(|reels| {
                        format!("\t\t\t\"{}\"", 
                            reels.instanses.iter().filter(|instanse| {instanse.correct}).last().map(|r| {
                                let count = if r.reel.len() > 5 { r.reel.len() - 2 } else { r.reel.len() };
                                r.reel.iter().take(count).map(|x| format!("{}", num_str_to_letter(&x.to_string()).unwrap_or("Z".to_string()))).collect::<Vec<_>>().join("")
                            }).unwrap_or_default()
                        )
                    }).collect::<Vec<String>>().join(",\n")
                )
            }).collect::<Vec<String>>().join(",\n")
        },
        //reelsBonus
        {
            let mut mech_blocks = Vec::new();
            {
                struct LenAgg {respins: i64,symbols: HashMap<i64, i64>}
                // 1) Собираем и агрегируем по mech_id → length_idx
                let mut agg: HashMap<String, Vec<LenAgg>> = HashMap::new();
                for cat in &a_categories.category {
                    for mech in &cat.bonus.by_mechanics {
                        let mech_id = mech.id.iter().map(|n| n.to_string()).collect::<String>();
                        let lens = agg.entry(mech_id.clone()).or_insert_with(|| {
                            mech.by_bonus_lenghts.iter().map(|_| LenAgg {respins: 0,symbols: HashMap::new()}).collect()
                        });
                        for (i, by_len) in mech.by_bonus_lenghts.iter().enumerate() {
                            let la = &mut lens[i];
                            la.respins += by_len.respins;
                            for sym in &by_len.symbols {
                                let sym_count: i64 = sym.count;
                                *la.symbols.entry(sym.id).or_insert(0) += sym_count;
                            }
                        }
                    }
                }
                // 2) Строим JSON-строку из агрегированных и отсортированных данных
                let mut mech_ids: Vec<String> = agg.keys().cloned().collect();
                mech_ids.sort_by_key(|id| id.parse::<i64>().unwrap_or(0));
                for mech_id in mech_ids {
                    let lengths = &agg[&mech_id];
                    let mut len_blocks = Vec::new();
                    for la in lengths.iter() {
                        let mut sym_entries = Vec::new();
                        let mut sids: Vec<i64> = la.symbols.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.symbols[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 * 100.0 / 15.0} else { 0.0 };
                            let repeat_count = (p_sym / 10.0).round() as usize;
                            let new_id = if a_appearing_symbols.contains(&sid) {16} else {sid};
                            let ch = num_str_to_letter(&new_id.to_string()).and_then(|s| s.chars().next()).unwrap_or('@');
                            sym_entries.push(std::iter::repeat(ch).take(repeat_count).collect::<String>());
                        }
                        let mut symbols_block = format!("{}", sym_entries.join(""));
                        if symbols_block.len() == 0 {symbols_block.push_str("AAA");}
                        else {if symbols_block.len() < 1000 {symbols_block.push_str(&std::iter::repeat('A').take(1000 - symbols_block.len()).collect::<String>());}}

                        len_blocks.push(format!("\t\t\t\"{}\"", symbols_block));
                    }
                    mech_blocks.push(format!("\t\t\"{}\":[\n{}\n\t\t]", mech_id, len_blocks.join(",\n")));
                }
            }
            {
                struct LenAgg {respins: i64,symbols: HashMap<i64, i64>}
                // 1) Собираем и агрегируем по mech_id → length_idx
                let mut agg: HashMap<String, Vec<LenAgg>> = HashMap::new();
                for cat in &a_categories.buy_category[0] {
                    for mech in &cat.bonus.by_mechanics {
                        let mech_id = mech.id.iter().map(|n| n.to_string()).collect::<String>();
                        let lens = agg.entry(mech_id.clone()).or_insert_with(|| {
                            mech.by_bonus_lenghts.iter().map(|_| LenAgg {respins: 0,symbols: HashMap::new()}).collect()
                        });
                        for (i, by_len) in mech.by_bonus_lenghts.iter().enumerate() {
                            let la = &mut lens[i];
                            la.respins += by_len.respins;
                            for sym in &by_len.symbols {
                                let sym_count: i64 = sym.count;
                                *la.symbols.entry(sym.id).or_insert(0) += sym_count;
                            }
                        }
                    }
                }
                // 2) Строим JSON-строку из агрегированных и отсортированных данных
                let mut mech_ids: Vec<String> = agg.keys().cloned().collect();
                mech_ids.sort_by_key(|id| id.parse::<i64>().unwrap_or(0));
                for mech_id in mech_ids {
                    let lengths = &agg[&mech_id];
                    let mut len_blocks = Vec::new();
                    for la in lengths.iter() {
                        let mut sym_entries = Vec::new();
                        let mut sids: Vec<i64> = la.symbols.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.symbols[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 * 100.0 / 15.0} else { 0.0 };
                            let repeat_count = (p_sym / 10.0).round() as usize;
                            let new_id = if a_appearing_symbols.contains(&sid) {16} else {sid};
                            let ch = num_str_to_letter(&new_id.to_string()).and_then(|s| s.chars().next()).unwrap_or('@');
                            sym_entries.push(std::iter::repeat(ch).take(repeat_count).collect::<String>());
                        }
                        let mut symbols_block = format!("{}", sym_entries.join(""));
                        if symbols_block.len() == 0 {symbols_block.push_str("AAA");}
                        else {if symbols_block.len() < 1000 {symbols_block.push_str(&std::iter::repeat('A').take(1000 - symbols_block.len()).collect::<String>());}}

                        len_blocks.push(format!("\t\t\t\"{}\"", symbols_block));
                    }
                    mech_blocks.push(format!("\t\t\"{}\":[\n{}\n\t\t]", mech_id, len_blocks.join(",\n")));
                }
            }
            {
                struct LenAgg {respins: i64,symbols: HashMap<i64, i64>}
                // 1) Собираем и агрегируем по mech_id → length_idx
                let mut agg: HashMap<String, Vec<LenAgg>> = HashMap::new();
                for cat in &a_categories.buy_category[1] {
                    for mech in &cat.bonus.by_mechanics {
                        let mech_id = mech.id.iter().map(|n| n.to_string()).collect::<String>();
                        let lens = agg.entry(mech_id.clone()).or_insert_with(|| {
                            mech.by_bonus_lenghts.iter().map(|_| LenAgg {respins: 0,symbols: HashMap::new()}).collect()
                        });
                        for (i, by_len) in mech.by_bonus_lenghts.iter().enumerate() {
                            let la = &mut lens[i];
                            la.respins += by_len.respins;
                            for sym in &by_len.symbols {
                                let sym_count: i64 = sym.count;
                                *la.symbols.entry(sym.id).or_insert(0) += sym_count;
                            }
                        }
                    }
                }
                // 2) Строим JSON-строку из агрегированных и отсортированных данных
                let mut mech_ids: Vec<String> = agg.keys().cloned().collect();
                mech_ids.sort_by_key(|id| id.parse::<i64>().unwrap_or(0));
                for mech_id in mech_ids {
                    let lengths = &agg[&mech_id];
                    let mut len_blocks = Vec::new();
                    for la in lengths.iter() {
                        let mut sym_entries = Vec::new();
                        let mut sids: Vec<i64> = la.symbols.keys().cloned().collect();
                        sids.sort();
                        for sid in sids {
                            let cnt = la.symbols[&sid];
                            let p_sym = if la.respins > 0 {cnt as f64 * 100.0 / la.respins as f64 * 100.0 / 15.0} else { 0.0 };
                            let repeat_count = (p_sym / 10.0).round() as usize;
                            let new_id = if a_appearing_symbols.contains(&sid) {16} else {sid};
                            let ch = num_str_to_letter(&new_id.to_string()).and_then(|s| s.chars().next()).unwrap_or('@');
                            sym_entries.push(std::iter::repeat(ch).take(repeat_count).collect::<String>());
                        }
                        let mut symbols_block = format!("{}", sym_entries.join(""));
                        if symbols_block.len() == 0 {symbols_block.push_str("AAA");}
                        else {if symbols_block.len() < 1000 {symbols_block.push_str(&std::iter::repeat('A').take(1000 - symbols_block.len()).collect::<String>());}}

                        len_blocks.push(format!("\t\t\t\"{}\"", symbols_block));
                    }
                    mech_blocks.push(format!("\t\t\"{}\":[\n{}\n\t\t]", mech_id, len_blocks.join(",\n")));
                }
            }

            // 3) Финальный JSON-блок
            format!("{}", mech_blocks.join(",\n"))
        },
        //lines
        {
            // 1) Собираем все шаблоны строк
            let mut lines = Vec::new();
            if let Value::Array(rows) = a_categories.settings.get("paylines").unwrap() {
                for row in rows {
                    if let Value::Array(cells) = row {
                        // строим строку из символов
                        let pattern: String = cells
                            .iter()
                            .map(|n| match n.as_i64().unwrap_or(-1) {
                                1 => '-',
                                0 => '^',
                                2 => '_',
                                _ => '?' 
                            })
                            .collect();
                        // пушим с отступом и кавычками
                        lines.push(format!("\t\t\"{}\"", pattern));
                    }
                }
            }
            // 2) Собираем финальную JSON-строку вручную через format!
            format!(
                "[\n{}\n\t]",
                lines.join(",\n")
            )
        },
        //wins
        {
            // 1) Собираем пары (буква → Vec<(occurrences, multiplier)>)
            let mut entries = Vec::new();
            if let Value::Object(m) = a_categories.settings.get("paytable").unwrap() {
                for (num_str, arr_val) in m {
                    if let Some(letter) = num_str_to_letter(num_str) {
                        if let Value::Array(entries_val) = arr_val {
                            // собираем внутреннюю карту
                            let mut inner = Vec::new();
                            for item in entries_val {
                                if let Value::Object(o) = item {
                                    if let (Some(mult), Some(occ)) = (
                                        o.get("multiplier").and_then(Value::as_i64),
                                        o.get("occurrences").and_then(Value::as_i64),
                                    ) {
                                        inner.push((occ, mult));
                                    }
                                }
                            }
                            entries.push((letter, inner));
                        }
                    }
                }
            }
            // 2) Формируем строки для каждого блока "A": { "3":2, "4":4, ... }
            let mut blocks = Vec::new();
            for (letter, inner) in entries {
                // для данного letter собираем строки `"occ":mult`
                let inner_lines: Vec<String> = inner
                    .iter()
                    .map(|&(occ, mult)| format!("\t\t\t\"{}\": {}", occ, mult))
                    .collect();
                // склейка через ",\n"
                let inner_body = inner_lines.join(",\n");
                // формируем блок
                let block = format!(
                    "\t\t\"{}\": {{\n{}\n\t\t}}",
                    letter,
                    inner_body
                );
                blocks.push(block);
            }
            // 3) Собираем весь JSON
            let body = blocks.join(",\n");
            let result = format!("{{\n{}\n\t}}", body);
            result
        },
    );
    if let Some(parent) = Path::new(&(a_location.to_owned() + &a_game_name.clone() + "/fugaso/fugaso.json")).parent() {let _ = fs::create_dir_all(parent);}
    fs::write(a_location.to_owned() + &a_game_name.clone() + "/fugaso/fugaso.json", format_reels).unwrap();
}

fn mechanics_name(mut ids: Vec<i64>) -> String {
    ids.sort();
    ids.dedup();
    ids.iter()
        .filter_map(|&id| match id {
            1 => Some("Add"),
            2 => Some("Twice"),
            3 => Some("Cluster"),
            _ => None, // игнорируем неизвестные
        })
        .collect::<String>()
}
fn mechanics_str_name(ids_str: &str) -> String {
    // Парсим каждый символ строки в цифру и превращаем её в i64
    let mut ids: Vec<i64> = ids_str
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i64))
        .collect();

    // Сортируем и убираем дубликаты
    ids.sort();
    ids.dedup();

    // Мапим на названия и собираем в одну строку
    ids.iter()
        .filter_map(|&id| match id {
            1 => Some("Add"),
            2 => Some("Twice"),
            3 => Some("Cluster"),
            _ => None,
        })
        .collect::<String>()
}

fn num_str_to_letter(s: &str) -> Option<String> {
    let n: usize = s.parse().ok()?;
    if (1..=26).contains(&n) {
        let c = (b'A' + (n as u8) - 1) as char;
        Some(c.to_string())
    } else {
        None
    }
}

pub fn fugaso_by_amount(a_transactions: &Vec<Value>, a_location: String, a_game_name: String) {
    let bins = [(0,    1), (1,    5),
        (5,    10),  (10,   50),  (50,  100),  (100, 200),  (200, 300),
        (300,  400), (400,  500), (500,  600), (600, 800),  (800, 1000),
        (1000, 2000),(2000, 3000),(3000, 5000),(5000, 6000),(6000, 7000),
        (7000, 8000),(8000, 9000),(9000,10000),(10000,11000),(11000,1000000),
    ];   
    let mut counts_s0 = vec![0; bins.len()];
    let mut counts_s1 = vec![0; bins.len()];
    let mut counts_s2 = vec![0; bins.len()];
    let mut counts_b0 = vec![0; bins.len()];
    let mut counts_b1 = vec![0; bins.len()];
    let mut counts_b2 = vec![0; bins.len()];
    let mut checking_list: Vec<String> = Vec::new();
    for transaction in a_transactions {
        if transaction.get("out").and_then(|response| response.get("command")).and_then(|command| command.as_str()) == Some("play") {
            if transaction.get("out").and_then(|response| response.get("status")).and_then(|status| status.get("code")).and_then(|code| code.as_str()) == Some("OK") { 
                if let Some(context) = transaction.get("out").and_then(|response| response.get("context")) {
                    
                    if context.get("current").and_then(|v| v.as_str()) == Some("spins") {
                        //checking
                        {
                            let board = context.get("spins").and_then(|spins| spins.get("board")).and_then(|board| board.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {
                                    array_inner.as_array().map(|value| {value.iter().filter_map(|v| {v.as_i64()}).collect::<Vec<i64>>()})
                            }).collect::<Vec<Vec<i64>>>()}).unwrap_or_default();
                            if board.iter().any(|column| {(column.contains(&11) && column.contains(&12)) || (column.contains(&12) && column.contains(&13)) || (column.contains(&13) && column.contains(&11))}) {
                                checking_list.push(format!("{{\"base_game_more_one_bonus_in_column\":{}}}", &transaction.to_string()));
                            }
                            /*if board.iter().flatten().filter(|&&x| x == 11).count() > 1 {
                                checking_list.push(format!("{{\"base_game_more_one_boost_in_board\":{}}}", &transaction.to_string()));
                            }
                            if board.iter().flatten().filter(|&&x| x == 12).count() > 1 {
                                checking_list.push(format!("{{\"base_game_more_one_double_in_board\":{}}}", &transaction.to_string()));
                            }
                            if board.iter().flatten().filter(|&&x| x == 13).count() > 1 {
                                checking_list.push(format!("{{\"base_game_more_one_collect_in_board\":{}}}", &transaction.to_string()));
                            }*/
                        }
                        let bet_per_line = context.get("last_args").and_then(|spins| spins.get("bet_per_line")).and_then(|v| v.as_i64()).unwrap_or_default();
                        let l_last_action = context.get("last_action").and_then(|v| v.as_str());
                        let l_selected_mode = context.get("spins").and_then(|spins| spins.get("selected_mode")).and_then(|v| v.as_str());
                        let round_win = context.get("spins").and_then(|spins| spins.get("round_win")).and_then(|v| v.as_i64()).unwrap_or_default();

                        let round_bet = 
                            if l_last_action == Some("spin") {
                                context.get("spins").and_then(|spins| spins.get("round_bet")).and_then(|v| v.as_i64()).unwrap_or_default()
                            } else if l_last_action == Some("buy_spin") {
                                if l_selected_mode == Some("1") {100 * 20 * bet_per_line}
                                else if l_selected_mode == Some("2") {300 * 20 *bet_per_line} 
                                else {0}
                            } else {0};
                            
                        if round_win > 0 {
                            for (i, (low, high)) in bins.iter().enumerate() {
                                if round_win >= round_bet * low && round_win < round_bet * high {
                                    if l_last_action == Some("spin") {
                                        counts_s0[i] += 1;
                                    } else if l_last_action == Some("buy_spin") {
                                        if l_selected_mode == Some("1") {counts_s1[i] += 1;}
                                        else if l_selected_mode == Some("2") {counts_s2[i] += 1;}
                                        else {continue;}
                                    } else {continue;}
                                    break;
                                }
                            }
                        }
                    }
                    else if context.get("current").and_then(|v| v.as_str()) == Some("bonus") {
                        //checkings
                        if context.get("last_action").and_then(|v| v.as_str()) == Some("bonus_init") {
                            let boost_values = context.get("bonus").and_then(|bonus| bonus.get("boost_values")).and_then(|boost_values| boost_values.as_array()).cloned().unwrap_or_default();
                            let double_values = context.get("bonus").and_then(|bonus| bonus.get("double_values")).and_then(|double_values| double_values.as_array()).cloned().unwrap_or_default();
                            let collect_values = context.get("bonus").and_then(|bonus| bonus.get("collect_values")).and_then(|collect_values| collect_values.as_array()).cloned().unwrap_or_default();
                            let mystery_values = context.get("bonus").and_then(|bonus| bonus.get("mystery_values")).and_then(|mystery_values| mystery_values.as_array()).cloned().unwrap_or_default();
                            let bs_count = context.get("bonus").and_then(|bonus| bonus.get("bs_count")).and_then(|v| v.as_i64()).unwrap_or_default() as usize;
                            
                            if mystery_values.len() > 0 {checking_list.push(format!("{{\"bonus_init_mystery_values\":{}}}", &transaction.to_string()));}
                            if bs_count > collect_values.len()+double_values.len()+boost_values.len()+5 {checking_list.push(format!("{{\"bonus_init_more_5\":{}}}", &transaction.to_string()));}


                        } else if context.get("last_action").and_then(|v| v.as_str()) == Some("respin") {
                            let new_bs = context.get("bonus").and_then(|bonus| bonus.get("new_bs")).and_then(|new_bs| new_bs.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {
                                    array_inner.as_array().map(|value| {value.iter().filter_map(|v| {v.as_i64()}).collect::<Vec<i64>>()})
                            }).collect::<Vec<Vec<i64>>>()}).unwrap_or_default();
                            let boost_values = context.get("bonus").and_then(|bonus| bonus.get("boost_values")).and_then(|boost_values| boost_values.as_array()).cloned().unwrap_or_default();
                            let double_values = context.get("bonus").and_then(|bonus| bonus.get("double_values")).and_then(|double_values| double_values.as_array()).cloned().unwrap_or_default();
                            let collect_values = context.get("bonus").and_then(|bonus| bonus.get("collect_values")).and_then(|collect_values| collect_values.as_array()).cloned().unwrap_or_default();
                            let mystery_values = context.get("bonus").and_then(|bonus| bonus.get("mystery_values")).and_then(|mystery_values| mystery_values.as_array()).cloned().unwrap_or_default();

                            
                            let mystery_count = mystery_values.iter().filter(|m| matches!(m.get("id").and_then(|id| id.as_i64()), Some(11) | Some(12) | Some(13))).count();

                            if (new_bs.len() - collect_values.len() - double_values.len() - boost_values.len() - (mystery_values.len() - mystery_count)) > 2 {checking_list.push(format!("{{\"coin_more_2_without_bonus_symbols_and_mystery\":{}}}", &transaction.to_string()));}
                            else if (new_bs.len() - collect_values.len() - double_values.len() - boost_values.len() - (mystery_values.len() - mystery_count)) > 1 {checking_list.push(format!("{{\"coin_more_1_without_bonus_symbols_and_mystery\":{}}}", &transaction.to_string()));}

                            if (new_bs.len() - mystery_values.len()) > 2 {checking_list.push(format!("{{\"coin_and_bonus_symbols_more_2_without_mystery\":{}}}", &transaction.to_string()));}
                            else if (new_bs.len() - mystery_values.len()) > 1 {checking_list.push(format!("{{\"coin_and_bonus_symbols_more_1_without_mystery\":{}}}", &transaction.to_string()));}


                            if mystery_values.len() > 2 {checking_list.push(format!("{{\"mystery_values_more_2\":{}}}", &transaction.to_string()));}
                            else if mystery_values.len() > 1 {checking_list.push(format!("{{\"mystery_values_more_1\":{}}}", &transaction.to_string()));}


                            if (collect_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(13)).count()) > 2 {checking_list.push(format!("{{\"collect_values_more_2_without_mystery\":{}}}", &transaction.to_string()));}
                            else if (collect_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(13)).count()) > 1 {checking_list.push(format!("{{\"collect_values_more_1_without_mystery\":{}}}", &transaction.to_string()));}

                            if collect_values.len() > 2 {checking_list.push(format!("{{\"collect_values_with_mystery_more_2\":{}}}", &transaction.to_string()));}
                            else if collect_values.len() > 1 {checking_list.push(format!("{{\"collect_values_with_mystery_more_1\":{}}}", &transaction.to_string()));}

                            if (double_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(12)).count()) > 2 {checking_list.push(format!("{{\"double_values_more_2_without_mystery\":{}}}", &transaction.to_string()));}
                            else if (double_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(12)).count()) > 1 {checking_list.push(format!("{{\"double_values_more_1_without_mystery\":{}}}", &transaction.to_string()));}

                            if double_values.len() > 2 {checking_list.push(format!("{{\"double_values_with_mystery_more_2\":{}}}", &transaction.to_string()));}
                            else if double_values.len() > 1 {checking_list.push(format!("{{\"double_values_with_mystery_more_1\":{}}}", &transaction.to_string()));}

                            if (boost_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(11)).count()) > 2 {checking_list.push(format!("{{\"boost_values_more_2_without_mystery\":{}}}", &transaction.to_string()));}
                            else if (boost_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(11)).count()) > 1 {checking_list.push(format!("{{\"boost_values_more_1_without_mystery\":{}}}", &transaction.to_string()));}

                            if boost_values.len() > 2 {checking_list.push(format!("{{\"boost_values_with_mystery_more_2\":{}}}", &transaction.to_string()));}
                            else if boost_values.len() > 1 {checking_list.push(format!("{{\"boost_values_with_mystery_more_1\":{}}}", &transaction.to_string()));}

                            
                            if ((boost_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(11)).count()) > 1) && ((double_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(12)).count()) > 1)
                                {checking_list.push(format!("{{\"boost_values_and_double_values_more_1_without_mystery\":{}}}", &transaction.to_string()));}
                            else if ((boost_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(11)).count()) > 0) && ((double_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(12)).count()) > 0)
                                {checking_list.push(format!("{{\"boost_values_and_double_values_without_mystery\":{}}}", &transaction.to_string()));}
                            else if (boost_values.len() > 0) && (double_values.len() > 0) {checking_list.push(format!("{{\"boost_values_and_double_values_with_mystery\":{}}}", &transaction.to_string()));}
                            
                            if ((boost_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(11)).count()) > 1) && ((collect_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(13)).count()) > 1)
                                {checking_list.push(format!("{{\"boost_values_and_collect_values_more_1_without_mystery\":{}}}", &transaction.to_string()));}
                            else if ((boost_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(11)).count()) > 0) && ((collect_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(13)).count()) > 0)
                                {checking_list.push(format!("{{\"boost_values_and_collect_values_without_mystery\":{}}}", &transaction.to_string()));}
                            else if (boost_values.len() > 0) && (collect_values.len() > 0) {checking_list.push(format!("{{\"boost_values_and_collect_values_with_mystery\":{}}}", &transaction.to_string()));}
                            
                            if ((collect_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(13)).count()) > 1) && ((double_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(12)).count()) > 1)
                                {checking_list.push(format!("{{\"collect_values_and_double_values_more_1_without_mystery\":{}}}", &transaction.to_string()));}
                            else if ((collect_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(13)).count()) > 0) && ((double_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(12)).count()) > 0)
                                {checking_list.push(format!("{{\"collect_values_and_double_values_without_mystery\":{}}}", &transaction.to_string()));}
                            else if (collect_values.len() > 0) && (double_values.len() > 0) {checking_list.push(format!("{{\"collect_values_and_double_values_with_mystery\":{}}}", &transaction.to_string()));}
                            
                            if ((collect_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(13)).count()) > 1)
                            && ((double_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(12)).count()) > 1)
                            && ((boost_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(11)).count()) > 1)
                                {checking_list.push(format!("{{\"collect_values_and_double_valuess_and_boost_values_more_1_without_mystery\":{}}}", &transaction.to_string()));}
                            else if ((collect_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(13)).count()) > 0)
                            && ((double_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(12)).count()) > 0)
                            && ((boost_values.len() - mystery_values.iter().filter(|m| m.get("id").and_then(|id| id.as_i64()) == Some(11)).count()) > 0)
                                {checking_list.push(format!("{{\"collect_values_and_double_values_and_boost_values_without_mystery\":{}}}", &transaction.to_string()));}
                            else if (collect_values.len() > 0) && (double_values.len() > 0) && (boost_values.len() > 0) {checking_list.push(format!("{{\"collect_values_and_double_values_and_boost_values_with_mystery\":{}}}", &transaction.to_string()));}

                        }
                        let round_bet = context.get("bonus").and_then(|bonus| bonus.get("round_bet")).and_then(|v| v.as_i64()).unwrap_or_default();
                        let round_win = context.get("bonus").and_then(|bonus| bonus.get("round_win")).and_then(|v| v.as_i64()).unwrap_or_default();
                        if round_win > 0 {
                            for (i, (low, high)) in bins.iter().enumerate() {
                                if round_win >= round_bet * low && round_win < round_bet * high {
                                    if context.get("bonus").and_then(|bonus| bonus.get("bonus_scenario")).and_then(|v| v.as_i64()) == Some(0) {
                                        counts_b0[i] += 1;
                                    } else if context.get("bonus").and_then(|spins| spins.get("bonus_scenario")).and_then(|v| v.as_i64()) == Some(1) {
                                        counts_b1[i] += 1;
                                    } else if context.get("bonus").and_then(|spins| spins.get("bonus_scenario")).and_then(|v| v.as_i64()) == Some(2) {
                                        counts_b2[i] += 1;
                                    } else {continue;}
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    checking_list.sort();
    let path = format!("{}/{}/fugaso/checkings.json", a_location, a_game_name);
    if let Some(parent) = Path::new(&path).parent() {let _ = fs::create_dir_all(parent);}
    fs::write(path, checking_list.join(",\n")).unwrap();

    let mut output = String::from("{\n");
    let sections = [
        ("spins",   &counts_s0),
        ("spins1",  &counts_s1),
        ("spins2",  &counts_s2),
        ("bonus",   &counts_b0),
        ("bonus1",  &counts_b1),
        ("bonus2",  &counts_b2),
    ];
    for (si, (name, counts)) in sections.iter().enumerate() {
        output.push_str(&format!("  \"{}\": {{\n", name));
        for (i, &(low, high)) in bins.iter().enumerate() {
            let comma = if i + 1 == bins.len() { "" } else { "," };
            output.push_str(&format!("    \"{}-{}\": {}{}\n", low, high, counts[i], comma));
        }
        let comma = if si + 1 == sections.len() { "" } else { "," };
        output.push_str(&format!("  }}{}\n", comma));
    }
    output.push_str("}\n");

    let path = format!("{}/{}/fugaso/by_amount.json", a_location, a_game_name);
    if let Some(dir) = Path::new(&path).parent() { let _ = fs::create_dir_all(dir); }
    fs::write(path, output).unwrap();

}