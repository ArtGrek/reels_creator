use serde_json::Value;
use std::time::{SystemTime, Duration};
use std::collections::HashSet;
use std::collections::BTreeMap;
use ordered_float::NotNan;
use indicatif::{ProgressBar, ProgressStyle};
use crate::bng::three_aztec_temples::models::{Game, Mode, Spins, Bet, Win, ByLine, BySymbol, ByLenght, Boards, UniqueBoardsInstanse, BoardsInstanse, Board, Bonus, ByMechanic, ByBonusLenght, Symbol, Col, Row, Reel, ReelInstanse, Multi};

pub fn extract(a_transactions: &Vec<Value>, a_game: &mut Game, a_spins_symbols: &Vec<i64>, a_appearing_symbols: &Vec<i64>, a_bonus_symbols: &Vec<i64>, a_bonus_symbol_values: &Vec<Multi>, a_mysterty_symbol: i64, a_buy_count: i64, width: usize, height: usize, ) {
    let pb_main = ProgressBar::new((a_transactions.len()) as u64);
    pb_main.set_prefix("Extracting data from transactions: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);

    let mut current_bs_count: usize = 0;
    let mut current_bonus_mechanic: Vec<i64> = Vec::new();
    let combo_symbols = set_default_combo_symbols(&a_appearing_symbols);
    let spins_symbols = set_default_symbols(&a_spins_symbols, &vec![], width, height);
    let bonus_symbols = set_default_symbols(&a_bonus_symbols, &a_bonus_symbol_values, width, height);
    let mystery_symbols = set_default_symbols(&a_bonus_symbols.iter().filter(|symbol| {**symbol != a_mysterty_symbol}).copied().collect::<Vec<i64>>(), &a_bonus_symbol_values, width, height);
    let bonus_lenghts = set_default_bonus_lenghts(&combo_symbols, &bonus_symbols, &mystery_symbols, width, height);
    let mechanics = set_default_mechanics(vec![1,2,3], &combo_symbols, &bonus_symbols, &mystery_symbols, &bonus_lenghts);
    for transaction in a_transactions {
        if transaction.get("out").and_then(|response| response.get("command")).and_then(|command| command.as_str()) == Some("play") {
            if transaction.get("out").and_then(|response| response.get("status")).and_then(|status| status.get("code")).and_then(|code| code.as_str()) == Some("OK") {
                if let Some(context) = transaction.get("out").and_then(|response| response.get("context")) {
                    
                    if a_categories.category.len() < (category_num+1) as usize {
                        let mut default_boards: Vec<BoardsInstanse> = vec![];
                        let mut unique_default_boards: Vec<UniqueBoardsInstanse> = vec![];
                        let mut default_reels: Vec<Reel> = vec![];
                        for _i in 0..width {
                            default_boards.push(BoardsInstanse { instanses: Vec::new() });
                            unique_default_boards.push(UniqueBoardsInstanse { count: 0, frequency_average: 0, instanses: Vec::new() });
                            default_reels.push(Reel { instanses: Vec::new() });
                        }
                        let default_category: Category = Category {
                            count: 0,
                            spins: Spins {
                                count: 0,
                                bet: Bet { count: 0, amount: 0 },
                                win: Win { count: 0, amount: 0, by_lines: Vec::new(), by_symbols: Vec::new() },
                                symbols: spins_symbols.clone(),
                            },
                            bonus: Bonus { appearances: 0, inits: 0, respins: 0, emerged: 0, amount: 0, reappearances: 0, combo_symbols: combo_symbols.clone(), reinits: 0, symbols: bonus_symbols.clone(), mystery_symbols: mystery_symbols.clone(), by_bonus_lenghts: bonus_lenghts.clone(), by_mechanics: mechanics.clone() },
                            boards: Boards { total: default_boards.clone(), filtered: default_boards.clone(), unique: unique_default_boards.clone(), multiplied: default_boards.clone() },
                            reels: default_reels.clone()
                        };
                        for _i in a_categories.category.len()..(category_num+1) as usize {
                            a_categories.category.push(default_category.clone());
                            for j in 0..a_buy_count as usize {
                                a_categories.buy_category[j].push(default_category.clone());
                            }
                        }
                    }

                    let l_category: &mut Category;
                    //spins
                    if context.get("current").and_then(|v| v.as_str()) == Some("spins") {
                        let l_last_action = context.get("last_action").and_then(|v| v.as_str());
                        let l_selected_mode = context.get("spins").and_then(|spins| spins.get("selected_mode")).and_then(|v| v.as_str());
                        if l_last_action == Some("spin") {l_category = &mut a_categories.category[category_num as usize];} else if l_last_action == Some("buy_spin") {
                            if l_selected_mode == Some("1") {l_category = &mut a_categories.buy_category[0][category_num as usize];} 
                            else if l_selected_mode == Some("2") {l_category = &mut a_categories.buy_category[1][category_num as usize];} 
                            else {continue;}
                        } else {continue;}
                        {    
                            //pars
                            let round_bet = if l_last_action == Some("spin") {context.get("spins").and_then(|spins| spins.get("round_bet")).and_then(|v| v.as_i64()).unwrap_or_default()}
                            else if l_last_action == Some("buy_spin") {
                                if l_selected_mode == Some("1") {a_categories.settings.get("buy_bonus_price").and_then(|v| v.as_array()).and_then(|arr| arr.get(0)).and_then(Value::as_i64).unwrap() * a_categories.settings.get("bet_factor").and_then(Value::as_array).and_then(|arr| arr.get(0)).and_then(Value::as_i64).unwrap_or(1)}
                                else if l_selected_mode == Some("2") {a_categories.settings.get("buy_bonus_price").and_then(|v| v.as_array()).and_then(|arr| arr.get(1)).and_then(Value::as_i64).unwrap() * a_categories.settings.get("bet_factor").and_then(Value::as_array).and_then(|arr| arr.get(0)).and_then(Value::as_i64).unwrap_or(1)}
                                else {0}
                            } else {0};
                            let round_win = context.get("spins").and_then(|spins| spins.get("round_win")).and_then(|v| v.as_i64()).unwrap_or_default();
                            let board = context.get("spins").and_then(|spins| spins.get("board")).and_then(|board| board.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {
                                    array_inner.as_array().map(|value| {value.iter().filter_map(|v| {v.as_i64()}).collect::<Vec<i64>>()})
                            }).collect::<Vec<Vec<i64>>>()}).unwrap_or_default();
                            let bs_values = context.get("spins").and_then(|bonus| bonus.get("bs_values")).and_then(|bs_values| bs_values.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {array_inner.as_array().map(|value| {value.iter().filter_map(|v| match v {
                                        Value::Number(n) => {
                                            if let Some(i) = n.as_i64() {Some(Multi::Int(i as i64))}
                                            else if let Some(f) = n.as_f64() {Some(Multi::Float(NotNan::new(f).unwrap()))}
                                            else {Some(Multi::Int(0))}
                                        }
                                        Value::String(s) => Some(Multi::String(s.clone())),
                                        _ => Some(Multi::Int(0))
                                }).collect::<Vec<Multi>>()})}).collect::<Vec<Vec<Multi>>>()
                            }).unwrap_or_default();
                            //collect
                            l_category.count += 1;
                            l_category.spins.count += 1;
                            l_category.spins.bet.count += 1;
                            l_category.spins.bet.amount += round_bet;
                            if round_win > 0 {
                                l_category.spins.win.count += 1;
                                l_category.spins.win.amount += round_win;
                            }
                            if let Some(winlines) = context.get("spins").and_then(|spins| spins.get("winlines")).and_then(|winlines| winlines.as_array()) {
                                winlines.iter().for_each(|winline| {
                                    let line = winline.get("line").and_then(|v| v.as_i64()).unwrap();
                                    let symbol = winline.get("symbol").and_then(|v| v.as_i64()).unwrap();
                                    let occurrences = winline.get("occurrences").and_then(|v| v.as_i64()).unwrap();
                                    let amount = winline.get("amount").and_then(|v| v.as_i64()).unwrap();
                                    if l_category.spins.win.by_lines.len() < line as usize {
                                        for i in l_category.spins.win.by_lines.len()..line as usize {
                                            l_category.spins.win.by_lines.push(ByLine { id: (i+1) as i64, count: 0, amount: 0, by_lengths: Vec::new() });
                                        }
                                    }
                                    if l_category.spins.win.by_lines[(line-1) as usize].by_lengths.len() < occurrences as usize {
                                        for i in l_category.spins.win.by_lines[(line-1) as usize].by_lengths.len()..occurrences as usize {
                                            l_category.spins.win.by_lines[(line-1) as usize].by_lengths.push(ByLenght { length: (i+1) as i64, count: 0, amount: 0 });
                                        }
                                    }
                                    if l_category.spins.win.by_symbols.len() < symbol as usize {
                                        for i in l_category.spins.win.by_symbols.len()..symbol as usize {
                                            l_category.spins.win.by_symbols.push(BySymbol { id: (i+1) as i64, count: 0, amount: 0, by_lengths: Vec::new() });
                                        }
                                    }
                                    if l_category.spins.win.by_symbols[(symbol-1) as usize].by_lengths.len() < occurrences as usize {
                                        for i in l_category.spins.win.by_symbols[(symbol-1) as usize].by_lengths.len()..occurrences as usize {
                                            l_category.spins.win.by_symbols[(symbol-1) as usize].by_lengths.push(ByLenght { length: (i+1) as i64, count: 0, amount: 0 });
                                        }
                                    }
                                    l_category.spins.win.by_lines[(line-1) as usize].count += 1;
                                    l_category.spins.win.by_lines[(line-1) as usize].amount += amount;
                                    l_category.spins.win.by_lines[(line-1) as usize].by_lengths[(occurrences-1) as usize].count += 1;
                                    l_category.spins.win.by_lines[(line-1) as usize].by_lengths[(occurrences-1) as usize].amount += amount;
                                    l_category.spins.win.by_symbols[(symbol-1) as usize].count += 1;
                                    l_category.spins.win.by_symbols[(symbol-1) as usize].amount += amount;
                                    l_category.spins.win.by_symbols[(symbol-1) as usize].by_lengths[(occurrences-1) as usize].count += 1;
                                    l_category.spins.win.by_symbols[(symbol-1) as usize].by_lengths[(occurrences-1) as usize].amount += amount;
                                });
                            }
                            let mut l_coin_count = 0;
                            let mut flags = (false, false, false);
                            board.iter().enumerate().for_each(|(col_num, column)| {column.iter().enumerate().for_each(|(row_num, symbol_id)| {
                                l_category.spins.symbols[(*symbol_id-1) as usize].count += 1;
                                l_category.spins.symbols[(*symbol_id-1) as usize].cols[col_num].count += 1;
                                l_category.spins.symbols[(*symbol_id-1) as usize].cols[col_num].rows[row_num].count += 1;
                                match *symbol_id {
                                    10 => l_coin_count += 1,
                                    11 => flags.0 = true,
                                    12 => flags.1 = true,
                                    13 => flags.2 = true,
                                    _  => {}
                                }
                            })});
                            if l_coin_count > 0 {
                                board.iter().enumerate().for_each(|(col_num, column)| {column.iter().enumerate().for_each(|(row_num, symbol_id)| {
                                    if *symbol_id == 10 {
                                        if let Some(idx) = l_category.bonus.by_bonus_lenghts[l_coin_count-1].symbols.iter().position(|s| s.id == 10) {
                                            l_category.bonus.by_bonus_lenghts[l_coin_count-1].symbols[idx].count += 1;
                                            l_category.bonus.by_bonus_lenghts[l_coin_count-1].symbols[idx].cols[col_num].count += 1;
                                            l_category.bonus.by_bonus_lenghts[l_coin_count-1].symbols[idx].cols[col_num].rows[row_num].count += 1;
                                            if let Some(value) = l_category.bonus.by_bonus_lenghts[l_coin_count-1].symbols[idx].values.get_mut(&bs_values[col_num][row_num]) {*value += 1;}
                                        }
                                    }
                                })});
                            }
                            if flags.0 || flags.1 || flags.2 {
                                l_category.bonus.appearances += 1;
                                match flags {
                                    (true, true, true) => {if let Some(idf) = l_category.bonus.by_mechanics.iter().position(|mechanic| mechanic.id == vec![1,2,3]) {l_category.bonus.by_mechanics[idf].appearances += 1;}},
                                    (false, true, true) => {if let Some(idf) = l_category.bonus.by_mechanics.iter().position(|mechanic| mechanic.id == vec![2,3]) {l_category.bonus.by_mechanics[idf].appearances += 1;}},
                                    (true, false, true) => {if let Some(idf) = l_category.bonus.by_mechanics.iter().position(|mechanic| mechanic.id == vec![1,3]) {l_category.bonus.by_mechanics[idf].appearances += 1;}},
                                    (true, true, false) => {if let Some(idf) = l_category.bonus.by_mechanics.iter().position(|mechanic| mechanic.id == vec![1,2]) {l_category.bonus.by_mechanics[idf].appearances += 1;}},
                                    (false, false, true) => {if let Some(idf) = l_category.bonus.by_mechanics.iter().position(|mechanic| mechanic.id == vec![3]) {l_category.bonus.by_mechanics[idf].appearances += 1;}},
                                    (true, false, false) => {if let Some(idf) = l_category.bonus.by_mechanics.iter().position(|mechanic| mechanic.id == vec![1]) {l_category.bonus.by_mechanics[idf].appearances += 1;}},
                                    (false, true, false) => {if let Some(idf) = l_category.bonus.by_mechanics.iter().position(|mechanic| mechanic.id == vec![2]) {l_category.bonus.by_mechanics[idf].appearances += 1;}},
                                    _  => {}
                                }
                            }
                            for i in 0..width {
                                l_category.boards.total[i].instanses.push(board[i].clone());
                            }
                            
                        } 
                    }
                    //bonus
                    else if context.get("current").and_then(|v| v.as_str()) == Some("bonus") {
                        let category_num = context.get("bonus").and_then(|bonus| bonus.get("bonus_game_type")).and_then(|v| v.as_i64()).map(|n| n - 1).unwrap_or(0);
                        if context.get("bonus").and_then(|spins| spins.get("selected_mode")).and_then(|v| v.as_str()) == Some("1") {
                            l_category = &mut a_categories.buy_category[0][category_num as usize];
                        } else if context.get("bonus").and_then(|spins| spins.get("selected_mode")).and_then(|v| v.as_str()) == Some("2") {
                            l_category = &mut a_categories.buy_category[1][category_num as usize];
                        } else {l_category = &mut a_categories.category[category_num as usize];} 
                        
                        {
                            //pars
                            let last_action = context.get("last_action").and_then(|v| v.as_str()).unwrap_or_default();
                            let round_bet = context.get("bonus").and_then(|bonus| bonus.get("round_bet")).and_then(|v| v.as_i64()).unwrap_or_default();
                            let round_win = context.get("bonus").and_then(|bonus| bonus.get("round_win")).and_then(|v| v.as_i64()).unwrap_or_default();
                            let bs_count = context.get("bonus").and_then(|bonus| bonus.get("bs_count")).and_then(|v| v.as_i64()).unwrap_or_default() as usize;
                            let bonus_mechanic = context.get("bonus").and_then(|bonus| bonus.get("bonus_mechanic")).and_then(|bonus_mechanic| bonus_mechanic.as_array()).map(|array| {array.iter().filter_map(|v| {v.as_i64()}).collect::<Vec<i64>>()}).unwrap_or_default();
                            let new_bs = context.get("bonus").and_then(|bonus| bonus.get("new_bs")).and_then(|new_bs| new_bs.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {
                                    array_inner.as_array().map(|value| {value.iter().filter_map(|v| {v.as_i64()}).collect::<Vec<i64>>()})
                            }).collect::<Vec<Vec<i64>>>()}).unwrap_or_default();
                            let origin_board = context.get("bonus").and_then(|bonus| bonus.get("origin_board")).and_then(|origin_board| origin_board.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {
                                    array_inner.as_array().map(|value| {value.iter().filter_map(|v| {v.as_i64()}).collect::<Vec<i64>>()})
                            }).collect::<Vec<Vec<i64>>>()});
                            let board = context.get("bonus").and_then(|bonus| bonus.get("board")).and_then(|board| board.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {
                                    array_inner.as_array().map(|value| {value.iter().filter_map(|v| {v.as_i64()}).collect::<Vec<i64>>()})
                            }).collect::<Vec<Vec<i64>>>()}).unwrap_or_default();
                            let origin_bs_v: Option<Vec<Vec<Multi>>> = context.get("bonus").and_then(|bonus| bonus.get("origin_bs_v")).and_then(|origin_bs_v| origin_bs_v.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {array_inner.as_array().map(|value| {value.iter().filter_map(|v| match v {
                                        Value::Number(n) => {
                                            if let Some(i) = n.as_i64() {
                                                let div = (i as f64) / (round_bet as f64);
                                                if div.fract() == 0.0 {Some(Multi::Int(div as i64))} else {Some(Multi::Float(NotNan::new(div).unwrap()))}
                                            }
                                            else if let Some(f) = n.as_f64() {
                                                let div = f / (round_bet as f64);
                                                if div.fract() == 0.0 {Some(Multi::Int(div as i64))} else {Some(Multi::Float(NotNan::new(div).unwrap()))}
                                            } else {Some(Multi::Int(0))}
                                        }
                                        Value::String(s) => Some(Multi::String(s.clone())),
                                        _ => Some(Multi::Int(0))
                                }).collect::<Vec<Multi>>()})}).collect::<Vec<Vec<Multi>>>()
                            });
                            let bs_v = context.get("bonus").and_then(|bonus| bonus.get("bs_v")).and_then(|bs_v| bs_v.as_array()).map(|array_outer| {
                                array_outer.iter().filter_map(|array_inner| {array_inner.as_array().map(|value| {value.iter().filter_map(|v| match v {
                                        Value::Number(n) => {
                                            if let Some(i) = n.as_i64() {
                                                let div = (i as f64) / (round_bet as f64);
                                                if div.fract() == 0.0 {Some(Multi::Int(div as i64))} else {Some(Multi::Float(NotNan::new(div).unwrap()))}
                                            }
                                            else if let Some(f) = n.as_f64() {
                                                let div = f / (round_bet as f64);
                                                if div.fract() == 0.0 {Some(Multi::Int(div as i64))} else {Some(Multi::Float(NotNan::new(div).unwrap()))}
                                            } else {Some(Multi::Int(0))}
                                        }
                                        Value::String(s) => Some(Multi::String(s.clone())),
                                        _ => Some(Multi::Int(0))
                                }).collect::<Vec<Multi>>()})}).collect::<Vec<Vec<Multi>>>()
                            }).unwrap_or_default();
                            //collect
                            l_category.count += 1;
                            if last_action == "bonus_init" {
                                l_category.bonus.inits += 1;
                                current_bonus_mechanic = bonus_mechanic.clone();
                                current_bs_count = bs_count;
                                current_bs_count -= 1;
                                //l_category.bonus.by_bonus_lenghts[current_bs_count-1].respins += 1;
                            }
                            else if last_action == "respin" {
                                l_category.bonus.respins += 1;
                                l_category.bonus.by_bonus_lenghts[current_bs_count-1].respins += 1;
                                if current_bs_count != bs_count {
                                    l_category.bonus.emerged += 1;
                                    l_category.bonus.by_bonus_lenghts[current_bs_count-1].emerged += 1;
                                }
                                l_category.bonus.amount += round_win;
                                if current_bonus_mechanic != bonus_mechanic {
                                    l_category.bonus.reinits += 1;
                                }
                            }
                            else if last_action == "bonus_spins_stop" {

                            }
                            let mut flags = (false, false, false);
                            for xy in new_bs.clone() {
                                if let Some(idx) = l_category.bonus.symbols.iter().position(|s| s.id == origin_board.as_ref().unwrap_or(&board)[xy[0] as usize][xy[1] as usize]) {
                                    l_category.bonus.symbols[idx].count += 1;
                                    l_category.bonus.symbols[idx].cols[xy[0] as usize].count += 1;
                                    l_category.bonus.symbols[idx].cols[xy[0] as usize].rows[xy[1] as usize].count += 1;
                                    if let Some(value) = l_category.bonus.symbols[idx].values.get_mut(&origin_bs_v.as_ref().unwrap_or(&bs_v)[xy[0] as usize][xy[1] as usize]) {*value += 1;}
                                    if l_category.bonus.symbols[idx].id == 13 {
                                        if let Some(mv_array) = context.get("bonus").and_then(|bonus| bonus.get("multi_values")).and_then(|mv| mv.as_array()) {
                                            let target_value: i64 = mv_array.iter().find(|item| {
                                                item.get("pos").and_then(|p| p.as_array()).and_then(|coords| {
                                                        let cx = coords.get(0).and_then(|v| v.as_u64())?;
                                                        let cy = coords.get(1).and_then(|v| v.as_u64())?;
                                                        if cx == xy[0] as u64 && cy == xy[1] as u64 {Some(())} else {None}
                                                }).is_some()
                                            }).and_then(|item| item.get("mult_value").and_then(|v| v.as_i64())).unwrap_or(0);
                                            if let Some(multiplayer) = l_category.bonus.symbols[idx].multiplayers.get_mut(&target_value){*multiplayer += 1;}
                                        }
                                    }
                                    if l_category.bonus.symbols[idx].id == a_mysterty_symbol {
                                        if let Some(idy) = l_category.bonus.mystery_symbols.iter().position(|s| s.id == origin_board.as_ref().unwrap_or(&board)[xy[0] as usize][xy[1] as usize]) {
                                            l_category.bonus.mystery_symbols[idy].count += 1;
                                            l_category.bonus.mystery_symbols[idy].cols[xy[0] as usize].count += 1;
                                            l_category.bonus.mystery_symbols[idy].cols[xy[0] as usize].rows[xy[1] as usize].count += 1;
                                            if let Some(value) = l_category.bonus.mystery_symbols[idy].values.get_mut(&origin_bs_v.as_ref().unwrap_or(&bs_v)[xy[0] as usize][xy[1] as usize]) {*value += 1;}
                                            if l_category.bonus.mystery_symbols[idy].id == 13 {
                                                if let Some(mv_array) = context.get("bonus").and_then(|bonus| bonus.get("multi_values")).and_then(|mv| mv.as_array()) {
                                                    let target_value: i64 = mv_array.iter().find(|item| {
                                                        item.get("pos").and_then(|p| p.as_array()).and_then(|coords| {
                                                                let cx = coords.get(0).and_then(|v| v.as_u64())?;
                                                                let cy = coords.get(1).and_then(|v| v.as_u64())?;
                                                                if cx == xy[0] as u64 && cy == xy[1] as u64 {Some(())} else {None}
                                                        }).is_some()
                                                    }).and_then(|item| item.get("mult_value").and_then(|v| v.as_i64())).unwrap_or(0);
                                                    if let Some(multiplayer) = l_category.bonus.mystery_symbols[idy].multiplayers.get_mut(&target_value){*multiplayer += 1;}
                                                }
                                            }
                                        }
                                    }
                                }
                                if let Some(idx) = l_category.bonus.by_bonus_lenghts[current_bs_count-1].symbols.iter().position(|s| s.id == origin_board.as_ref().unwrap_or(&board)[xy[0] as usize][xy[1] as usize]) {
                                    l_category.bonus.by_bonus_lenghts[current_bs_count-1].symbols[idx].count += 1;
                                    l_category.bonus.by_bonus_lenghts[current_bs_count-1].symbols[idx].cols[xy[0] as usize].count += 1;
                                    l_category.bonus.by_bonus_lenghts[current_bs_count-1].symbols[idx].cols[xy[0] as usize].rows[xy[1] as usize].count += 1;
                                    if let Some(value) = l_category.bonus.by_bonus_lenghts[current_bs_count-1].symbols[idx].values.get_mut(&origin_bs_v.as_ref().unwrap_or(&bs_v)[xy[0] as usize][xy[1] as usize]) {*value += 1;}
                                    if l_category.bonus.by_bonus_lenghts[current_bs_count-1].symbols[idx].id == 13 {
                                        if let Some(mv_array) = context.get("bonus").and_then(|bonus| bonus.get("multi_values")).and_then(|mv| mv.as_array()) {
                                            let target_value: i64 = mv_array.iter().find(|item| {
                                                item.get("pos").and_then(|p| p.as_array()).and_then(|coords| {
                                                        let cx = coords.get(0).and_then(|v| v.as_u64())?;
                                                        let cy = coords.get(1).and_then(|v| v.as_u64())?;
                                                        if cx == xy[0] as u64 && cy == xy[1] as u64 {Some(())} else {None}
                                                }).is_some()
                                            }).and_then(|item| item.get("mult_value").and_then(|v| v.as_i64())).unwrap_or(0);
                                            if let Some(multiplayer) = l_category.bonus.by_bonus_lenghts[current_bs_count-1].symbols[idx].multiplayers.get_mut(&target_value){*multiplayer += 1;}
                                        }
                                    }
                                    if l_category.bonus.by_bonus_lenghts[current_bs_count-1].symbols[idx].id == a_mysterty_symbol {
                                        if let Some(idy) = l_category.bonus.by_bonus_lenghts[current_bs_count-1].mystery_symbols.iter().position(|s| s.id == origin_board.as_ref().unwrap_or(&board)[xy[0] as usize][xy[1] as usize]) {
                                            l_category.bonus.by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].count += 1;
                                            l_category.bonus.by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].cols[xy[0] as usize].count += 1;
                                            l_category.bonus.by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].cols[xy[0] as usize].rows[xy[1] as usize].count += 1;
                                            if let Some(value) = l_category.bonus.by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].values.get_mut(&origin_bs_v.as_ref().unwrap_or(&bs_v)[xy[0] as usize][xy[1] as usize]) {*value += 1;}
                                            if l_category.bonus.by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].id == 13 {
                                                if let Some(mv_array) = context.get("bonus").and_then(|bonus| bonus.get("multi_values")).and_then(|mv| mv.as_array()) {
                                                    let target_value: i64 = mv_array.iter().find(|item| {
                                                        item.get("pos").and_then(|p| p.as_array()).and_then(|coords| {
                                                                let cx = coords.get(0).and_then(|v| v.as_u64())?;
                                                                let cy = coords.get(1).and_then(|v| v.as_u64())?;
                                                                if cx == xy[0] as u64 && cy == xy[1] as u64 {Some(())} else {None}
                                                        }).is_some()
                                                    }).and_then(|item| item.get("mult_value").and_then(|v| v.as_i64())).unwrap_or(0);
                                                    if let Some(multiplayer) = l_category.bonus.by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].multiplayers.get_mut(&target_value){*multiplayer += 1;}
                                                }
                                            }
                                        }
                                    }
                                }
                                match board[xy[0] as usize][xy[1] as usize] {
                                    11 => flags.0 = true,
                                    12 => flags.1 = true,
                                    13 => flags.2 = true,
                                    _  => {}
                                }
                            }
                            if flags.0 || flags.1 || flags.2 {
                                l_category.bonus.reappearances += 1;
                                let key: Vec<i64> = match flags {
                                    (true, true, true) => vec![11,12,13],
                                    (false, true, true) => vec![12,13],
                                    (true, false, true) => vec![11,13],
                                    (true, true, false) => vec![11,12],
                                    (false, false, true) => vec![13],
                                    (true, false, false) => vec![11],
                                    (false, true, false) => vec![12],
                                    _  => {vec![]}
                                };
                                if let Some(value) = l_category.bonus.combo_symbols.get_mut(&key) {*value += 1;}
                                if let Some(value) = l_category.bonus.by_bonus_lenghts[current_bs_count-1].combo_symbols.get_mut(&key) {*value += 1;}
                            }
                            //by mechanic
                            if let Some(idm) = l_category.bonus.by_mechanics.iter().position(|mechanic| mechanic.id == current_bonus_mechanic) {
                                for xy in new_bs.clone() {
                                    if let Some(idx) = l_category.bonus.by_mechanics[idm].symbols.iter().position(|s| s.id == origin_board.as_ref().unwrap_or(&board)[xy[0] as usize][xy[1] as usize]) {
                                        l_category.bonus.by_mechanics[idm].symbols[idx].count += 1;
                                        l_category.bonus.by_mechanics[idm].symbols[idx].cols[xy[0] as usize].count += 1;
                                        l_category.bonus.by_mechanics[idm].symbols[idx].cols[xy[0] as usize].rows[xy[1] as usize].count += 1;
                                        if let Some(value) = l_category.bonus.by_mechanics[idm].symbols[idx].values.get_mut(&origin_bs_v.as_ref().unwrap_or(&bs_v)[xy[0] as usize][xy[1] as usize]) {*value += 1;}
                                        if l_category.bonus.by_mechanics[idm].symbols[idx].id == 13 {
                                            if let Some(mv_array) = context.get("bonus").and_then(|bonus| bonus.get("multi_values")).and_then(|mv| mv.as_array()) {
                                                let target_value: i64 = mv_array.iter().find(|item| {
                                                    item.get("pos").and_then(|p| p.as_array()).and_then(|coords| {
                                                            let cx = coords.get(0).and_then(|v| v.as_u64())?;
                                                            let cy = coords.get(1).and_then(|v| v.as_u64())?;
                                                            if cx == xy[0] as u64 && cy == xy[1] as u64 {Some(())} else {None}
                                                    }).is_some()
                                                }).and_then(|item| item.get("mult_value").and_then(|v| v.as_i64())).unwrap_or(0);
                                                if let Some(multiplayer) = l_category.bonus.by_mechanics[idm].symbols[idx].multiplayers.get_mut(&target_value){*multiplayer += 1;}
                                            }
                                        }
                                        if l_category.bonus.symbols[idx].id == a_mysterty_symbol {
                                            if let Some(idy) = l_category.bonus.by_mechanics[idm].mystery_symbols.iter().position(|s| s.id == origin_board.as_ref().unwrap_or(&board)[xy[0] as usize][xy[1] as usize]) {
                                                l_category.bonus.by_mechanics[idm].mystery_symbols[idy].count += 1;
                                                l_category.bonus.by_mechanics[idm].mystery_symbols[idy].cols[xy[0] as usize].count += 1;
                                                l_category.bonus.by_mechanics[idm].mystery_symbols[idy].cols[xy[0] as usize].rows[xy[1] as usize].count += 1;
                                                if let Some(value) = l_category.bonus.by_mechanics[idm].mystery_symbols[idy].values.get_mut(&origin_bs_v.as_ref().unwrap_or(&bs_v)[xy[0] as usize][xy[1] as usize]) {*value += 1;}
                                                if l_category.bonus.by_mechanics[idm].mystery_symbols[idy].id == 13 {
                                                    if let Some(mv_array) = context.get("bonus").and_then(|bonus| bonus.get("multi_values")).and_then(|mv| mv.as_array()) {
                                                        let target_value: i64 = mv_array.iter().find(|item| {
                                                            item.get("pos").and_then(|p| p.as_array()).and_then(|coords| {
                                                                    let cx = coords.get(0).and_then(|v| v.as_u64())?;
                                                                    let cy = coords.get(1).and_then(|v| v.as_u64())?;
                                                                    if cx == xy[0] as u64 && cy == xy[1] as u64 {Some(())} else {None}
                                                            }).is_some()
                                                        }).and_then(|item| item.get("mult_value").and_then(|v| v.as_i64())).unwrap_or(0);
                                                        if let Some(multiplayer) = l_category.bonus.by_mechanics[idm].mystery_symbols[idy].multiplayers.get_mut(&target_value){*multiplayer += 1;}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if let Some(idx) = l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].symbols.iter().position(|s| s.id == origin_board.as_ref().unwrap_or(&board)[xy[0] as usize][xy[1] as usize]) {
                                        l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].symbols[idx].count += 1;
                                        l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].symbols[idx].cols[xy[0] as usize].count += 1;
                                        l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].symbols[idx].cols[xy[0] as usize].rows[xy[1] as usize].count += 1;
                                        if let Some(value) = l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].symbols[idx].values.get_mut(&origin_bs_v.as_ref().unwrap_or(&bs_v)[xy[0] as usize][xy[1] as usize]) {*value += 1;}
                                        if l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].symbols[idx].id == 13 {
                                            if let Some(mv_array) = context.get("bonus").and_then(|bonus| bonus.get("multi_values")).and_then(|mv| mv.as_array()) {
                                                let target_value: i64 = mv_array.iter().find(|item| {
                                                    item.get("pos").and_then(|p| p.as_array()).and_then(|coords| {
                                                            let cx = coords.get(0).and_then(|v| v.as_u64())?;
                                                            let cy = coords.get(1).and_then(|v| v.as_u64())?;
                                                            if cx == xy[0] as u64 && cy == xy[1] as u64 {Some(())} else {None}
                                                    }).is_some()
                                                }).and_then(|item| item.get("mult_value").and_then(|v| v.as_i64())).unwrap_or(0);
                                                if let Some(multiplayer) = l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].symbols[idx].multiplayers.get_mut(&target_value){*multiplayer += 1;}
                                            }
                                        }
                                        if l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].symbols[idx].id == a_mysterty_symbol {
                                            if let Some(idy) = l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].mystery_symbols.iter().position(|s| s.id == origin_board.as_ref().unwrap_or(&board)[xy[0] as usize][xy[1] as usize]) {
                                                l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].count += 1;
                                                l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].cols[xy[0] as usize].count += 1;
                                                l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].cols[xy[0] as usize].rows[xy[1] as usize].count += 1;
                                                if let Some(value) = l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].values.get_mut(&origin_bs_v.as_ref().unwrap_or(&bs_v)[xy[0] as usize][xy[1] as usize]) {*value += 1;}
                                                if l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].id == 13 {
                                                    if let Some(mv_array) = context.get("bonus").and_then(|bonus| bonus.get("multi_values")).and_then(|mv| mv.as_array()) {
                                                        let target_value: i64 = mv_array.iter().find(|item| {
                                                            item.get("pos").and_then(|p| p.as_array()).and_then(|coords| {
                                                                    let cx = coords.get(0).and_then(|v| v.as_u64())?;
                                                                    let cy = coords.get(1).and_then(|v| v.as_u64())?;
                                                                    if cx == xy[0] as u64 && cy == xy[1] as u64 {Some(())} else {None}
                                                            }).is_some()
                                                        }).and_then(|item| item.get("mult_value").and_then(|v| v.as_i64())).unwrap_or(0);
                                                        if let Some(multiplayer) = l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].mystery_symbols[idy].multiplayers.get_mut(&target_value){*multiplayer += 1;}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    match board[xy[0] as usize][xy[1] as usize] {
                                        11 => flags.0 = true,
                                        12 => flags.1 = true,
                                        13 => flags.2 = true,
                                        _  => {}
                                    }
                                }
                                if flags.0 || flags.1 || flags.2 {
                                    l_category.bonus.by_mechanics[idm].reappearances += 1;
                                    let key: Vec<i64> = match flags {
                                        (true, true, true) => vec![11,12,13],
                                        (false, true, true) => vec![12,13],
                                        (true, false, true) => vec![11,13],
                                        (true, true, false) => vec![11,12],
                                        (false, false, true) => vec![13],
                                        (true, false, false) => vec![11],
                                        (false, true, false) => vec![12],
                                        _  => {vec![]}
                                    };
                                    if let Some(value) = l_category.bonus.by_mechanics[idm].combo_symbols.get_mut(&key) {*value += 1;}
                                    if let Some(value) = l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].combo_symbols.get_mut(&key) {*value += 1;}
                                }
                                if last_action == "bonus_init" {
                                    l_category.bonus.by_mechanics[idm].inits += 1;
                                    //l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].respins += 1;
                                    current_bs_count += 1;
                                }
                                else if last_action == "respin" {
                                    l_category.bonus.by_mechanics[idm].respins += 1;
                                    l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].respins += 1;
                                    if current_bs_count != bs_count {
                                        l_category.bonus.by_mechanics[idm].emerged += 1;
                                        l_category.bonus.by_mechanics[idm].by_bonus_lenghts[current_bs_count-1].emerged += 1;
                                        current_bs_count = bs_count;
                                    }
                                    l_category.bonus.by_mechanics[idm].amount += round_win;
                                    if current_bonus_mechanic != bonus_mechanic {
                                        l_category.bonus.by_mechanics[idm].reinits += 1;
                                        current_bonus_mechanic = bonus_mechanic.clone();
                                    }
                                }
                                else if last_action == "bonus_spins_stop" {

                                }
                            }
                        }
                    }
                }
            }
        } else if transaction.get("out").and_then(|response| response.get("command")).and_then(|command| command.as_str()) == Some("start") {
            if transaction.get("out").and_then(|response| response.get("status")).and_then(|status| status.get("code")).and_then(|code| code.as_str()) == Some("OK") {
                if let Some(settings) = transaction.get("out").and_then(|response| response.get("settings")) {a_game.settings = settings.clone();}
            }
        }
        pb_main.inc(1);
    }
    pb_main.finish_with_message(" -> extracted");
}

pub fn set_default_symbols(a_symbols: &Vec<i64>, a_bonus_symbol_values: &Vec<Multi>, width: usize, height: usize) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = vec![];
    for symbol in a_symbols {
        let mut multiplayers: BTreeMap<i64, i64> =  BTreeMap::new();
        if *symbol == 13 {} else {
            multiplayers.insert(2, 0 );
            multiplayers.insert(3, 0 );
            multiplayers.insert(5, 0 );
        }
        let mut values: BTreeMap<Multi, i64> = BTreeMap::new();
        a_bonus_symbol_values.iter().for_each(|a_bonus_symbol_value| {values.insert(a_bonus_symbol_value.clone(), 0 );});
        let mut rows: Vec<Row> = Vec::new();
        for _i in 0..height {rows.push(Row { count: 0 })}
        let mut cols: Vec<Col> = Vec::new();
        for _i in 0..width {cols.push(Col { count: 0, rows: rows.clone() });}
        symbols.push(Symbol {id: *symbol, count: 0, values: values, multiplayers: multiplayers, cols: cols.clone()});
    };
    symbols
}

pub fn set_default_combo_symbols(a_symbols: &Vec<i64>) -> BTreeMap<Vec<i64>, i64> {
    let mut symbols: BTreeMap<Vec<i64>, i64> = BTreeMap::new();
    let n = a_symbols.len();
    // Пробежим по всем маскам от 1 до 2^n - 1 (чтобы получить все непустые подмножества)
    for mask in 1..(1 << n) {
        let mut combo = Vec::new();
        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                combo.push(a_symbols[i]);
            }
        }
        symbols.insert(combo, 0);
    }
    symbols
}

pub fn set_default_bonus_lenghts(a_combo_symbols: &BTreeMap<Vec<i64>, i64>, a_bonus_symbols: &Vec<Symbol>, a_mystery_symbols: &Vec<Symbol>, width: usize, height: usize) -> Vec<ByBonusLenght> {
    let mut bonus_lenghts: Vec<ByBonusLenght> = vec![];
    for _i in 0..(width*height) {
        bonus_lenghts.push(ByBonusLenght { respins: 0, emerged: 0, combo_symbols: a_combo_symbols.clone(), symbols: a_bonus_symbols.clone(), mystery_symbols: a_mystery_symbols.clone() });
    }
    bonus_lenghts
}

pub fn set_default_mechanics(a_ids: Vec<i64>, a_combo_symbols: &BTreeMap<Vec<i64>, i64>, a_bonus_symbols: &Vec<Symbol>, a_mystery_symbols: &Vec<Symbol>, a_bonus_lenghts: &Vec<ByBonusLenght>) -> Vec<ByMechanic> {
    let mut mechanics: Vec<ByMechanic> = vec![];
    let n = a_ids.len();
    // Пробежим по всем маскам от 1 до 2^n - 1 (чтобы получить все непустые подмножества)
    for mask in 1..(1 << n) {
        let mut combo = Vec::new();
        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                combo.push(a_ids[i]);
            }
        }
        mechanics.push(ByMechanic { id: combo.clone(), appearances: 0, inits: 0, respins: 0, emerged: 0, amount: 0, reappearances: 0, combo_symbols: a_combo_symbols.clone(), reinits: 0, symbols: a_bonus_symbols.clone(), mystery_symbols: a_mystery_symbols.clone(), by_bonus_lenghts: a_bonus_lenghts.clone() });
    }
    // Сортируем сначала по длине, потом лексикографически
    mechanics.sort_by(|a, b| {
        b.id.len()
         .cmp(&a.id.len())
         .then_with(|| b.id.cmp(&a.id))
    });
    mechanics
}

pub fn delete_boards_with_symbols(a_boards: Vec<Vec<i64>>, symbols: Vec<i64>) -> Vec<Vec<i64>> {
    let pb_main = ProgressBar::new((a_boards.len()) as u64);
    pb_main.set_prefix("Deleting appering symbols boards: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_boards: Vec<Vec<i64>> = a_boards.clone();
    if l_boards.len() > 0 {
        l_boards.retain(|board| {pb_main.inc(1); !board.iter().any(|symbol| symbols.contains(symbol))});
    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> remain ".to_owned() + &l_boards.len().to_string() + " boards");
    l_boards
}

pub fn select_unique_boards(a_boards: Vec<Vec<i64>>) -> UniqueBoardsInstanse {
    let pb_main = ProgressBar::new((a_boards.len()) as u64);
    pb_main.set_prefix("Finding unique boards: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_boards: UniqueBoardsInstanse = UniqueBoardsInstanse{ count: 0, frequency_average: 0, instanses: Vec::new() };
    if a_boards.len() > 0 {
        let mut hash_boards: HashSet<Vec<i64>> = HashSet::new();
        for board_num in 0..a_boards.len() {
            if hash_boards.insert(a_boards[board_num].clone()) {
                l_boards.instanses.push(Board { count:0, board: a_boards[board_num].clone() });
            }
            let pos = l_boards.instanses.iter().position(|x| x.board == a_boards[board_num]).unwrap();
            l_boards.count += 1;
            l_boards.instanses[pos].count += 1;
            pb_main.inc(1);
        }
    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> finded ".to_owned() + &l_boards.instanses.len().to_string() + " boards");
    l_boards
}

pub fn multiply_unique_boards_by_frequency(a_boards_filtered: Vec<Vec<i64>>, boards_unique: &mut UniqueBoardsInstanse, height: usize) -> Vec<Vec<i64>> {
    let pb_main = ProgressBar::new((boards_unique.instanses.len()) as u64);
    pb_main.set_prefix("Multipling unique boards: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_boards: Vec<Vec<i64>> = Vec::new();
    match a_boards_filtered.len() {
        10000..=usize::MAX => {
            let mut l_counts: Vec<i64> = boards_unique.instanses.iter().map(|x| x.count).collect();

            l_counts.sort();
            boards_unique.frequency_average = *l_counts.iter().min().unwrap();
            l_counts.iter().for_each(|occur_count| {
                if (*occur_count as f64 / boards_unique.frequency_average as f64).round() == 1.0 {
                    let temp_boards_frequency_average = ((boards_unique.frequency_average + occur_count) as f64 / 2.0).round() as i64;
                    boards_unique.frequency_average = temp_boards_frequency_average;
                }
            });
            for board_unique_num in 0..boards_unique.instanses.len() {
                let mut l_count = (boards_unique.instanses[board_unique_num].count as f64 / boards_unique.frequency_average as f64).round() as i64;
                if l_count == 0 {l_count = 1;}
                for _i in 0..l_count {
                    l_boards.push(boards_unique.instanses[board_unique_num].board.clone());
                }
                pb_main.inc(1);
            }
        },
        1..10000 => { 
            l_boards = boards_unique.instanses.iter().map(|x| x.board.clone()).collect();
            let temp_boards = l_boards.clone();
            loop {
                let mut inserted = false;
                for temp_board in temp_boards.iter() {
                    let mut unique_left: HashSet<Vec<i64>> = Default::default();
                    let mut unique_right: HashSet<Vec<i64>> = Default::default();
                    let l_left_count = l_boards.iter().filter(|board| {
                        if board[..(height-1) as usize] == temp_board[1..] && *board != temp_board && (!temp_board[1..].iter().all(|symbol| *symbol == temp_board[1]) || temp_board.iter().all(|symbol| *symbol == temp_board[0]) || board.iter().all(|symbol| *symbol == board[0])) {
                            unique_left.insert((*board).clone())
                        } else {false}
                    }).count();
                    let l_right_count = l_boards.iter().filter(|board| {
                        if board[1..] == temp_board[..(height-1) as usize] && *board != temp_board && (!temp_board[..(height-1) as usize].iter().all(|symbol| *symbol == temp_board[0]) || temp_board.iter().all(|symbol| *symbol == temp_board[0]) || board.iter().all(|symbol| *symbol == board[0])) {
                            unique_right.insert((*board).clone())
                        } else {false}
                    }).count();
                    let l_count = std::cmp::max(l_left_count, l_right_count);
                    let l_exist_count = l_boards.iter().filter(|board| {*board == temp_board}).count();
                    if l_count > l_exist_count {
                        inserted = true;
                        for _i in 0..l_count-l_exist_count {
                            l_boards.push(temp_board.clone());
                        }
                    }
                    pb_main.inc(1);
                } 
                if !inserted {break;}     
            }
        },
        _ => {l_boards = boards_unique.instanses.iter().map(|x| {pb_main.inc(1); x.board.clone()}).collect();}
    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> result ".to_owned() + &l_boards.len().to_string() + " boards");
    l_boards
}

pub fn _collect_reels(a_boards: &Vec<Vec<i64>>, height: usize, can_skip_reel_collect_after_timeout: bool, skip_collect_timeout_sec: u64) -> Vec<ReelInstanse> {
    let pb_main = ProgressBar::new((a_boards.len()) as u64);
    pb_main.set_prefix("Collecting reels: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_reels: Vec<ReelInstanse> = Vec::new();
    if a_boards.len() > 0 {
        let mut temp_boards: Vec<Vec<i64>> = a_boards.clone();
        let temp_board: Vec<i64> = temp_boards.remove(0);
        let temp_reel: Vec<i64> = temp_board;
        l_reels.push(ReelInstanse {reel: temp_reel, remaining: temp_boards, correct: false});
        let collect_start_time = SystemTime::now();
        for _i in 0..a_boards.len()-1 {
            if l_reels.len() > 0 {
                let mut stop_pos = l_reels.len()-1;
                let start_pos = 0;
                loop {
                    if l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-1] != -2 {
                        let mut pushed = false;
                        for board_pos in (0..l_reels[stop_pos].remaining.len()).rev() {
                            if l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-(height-1) as usize..] == l_reels[stop_pos].remaining[board_pos][..(height-1) as usize] 
                            && (false || !(
                                (2..height as usize).contains(&l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - height as usize ..].iter().rev().take_while(|&&x| {x == l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - 1]}).count())
                                && l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-1] != l_reels[stop_pos].remaining[board_pos][l_reels[stop_pos].remaining[board_pos].len()-1]
                            ))
                            && !(
                                l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - height as usize ..].iter().rev().filter(|&&x| {x == l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - 1]}).count() == height as usize
                                && l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-1] == l_reels[stop_pos].remaining[board_pos][l_reels[stop_pos].remaining[board_pos].len()-1]
                            )
                            {
                                let mut temp_boards = l_reels[stop_pos].remaining.clone();
                                temp_boards.remove(board_pos);
                                let mut temp_reel = l_reels[stop_pos].reel.clone();
                                let new_board = l_reels[stop_pos].remaining[board_pos][(height-1) as usize..].to_vec().clone();
                                temp_reel.extend_from_slice(&new_board);
                                l_reels.push(ReelInstanse {reel: temp_reel.clone(), remaining: temp_boards.clone(), correct: false});
                                pushed = true;
                            }
                        }
                        if pushed {l_reels.remove(stop_pos);} else {l_reels[stop_pos].reel.push(-2);}
                    } else {/*remove*/}
                    if start_pos == stop_pos {break;} stop_pos -= 1; 
                }
                let mut seen: HashSet<ReelInstanse> = HashSet::new(); let mut unique_reels: Vec<ReelInstanse> = Vec::new();
                for l_reel in l_reels.iter() {if seen.insert(l_reel.clone()) {unique_reels.push(l_reel.clone());} }
                l_reels = unique_reels.clone();
            } else {break;}
            if can_skip_reel_collect_after_timeout {if SystemTime::now() > collect_start_time + Duration::from_secs(skip_collect_timeout_sec as u64) {break;}}
            pb_main.inc(1);
        }
    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> collect ".to_owned() + &l_reels.len().to_string() + " reels");
    l_reels
}

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use tempfile::TempDir;

pub fn _collect_reels_v2(a_boards: &Vec<Vec<i64>>, height: usize, can_skip_reel_collect_after_timeout: bool, skip_collect_timeout_sec: u64) -> Vec<ReelInstanse> {
    let pb = ProgressBar::new(a_boards.len() as u64);
    pb.set_prefix("Collecting reels: ");
    pb.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    // создаём временную папку, в которой будут файлы с ReelInstanse
    let tmpdir = TempDir::new().expect("failed to create temp dir");
    let mut paths: Vec<PathBuf> = Vec::new();
    // инициализируем первую ветку
    if !a_boards.is_empty() {
        let mut rem = a_boards.to_vec();
        let first = rem.remove(0);
        let inst = ReelInstanse { reel: first, remaining: rem, correct: false };
        let p = tmpdir.path().join("0.json");
        let f = File::create(&p).unwrap();
        serde_json::to_writer(BufWriter::new(f), &inst).unwrap();
        paths.push(p);
    }
    let start = SystemTime::now();
    let total_iters = a_boards.len().saturating_sub(1);
    for iter in 0..total_iters {
        // если нечего обрабатывать — выход
        if paths.is_empty() { break; }
        // берём последнюю созданную ветку
        let idx = paths.len() - 1;
        let path = paths[idx].clone();
        let file = File::open(&path).unwrap();
        let mut li: ReelInstanse = serde_json::from_reader(BufReader::new(file)).unwrap();
        // если уже помечено «откатом», пропускаем
        if *li.reel.last().unwrap() != -2 {
            let mut pushed = false;
            // пробегаем все оставшиеся доски в обратном порядке
            for bpos in (0..li.remaining.len()).rev() {
                let suffix = &li.reel[li.reel.len() - (height-1)..];
                let head = &li.remaining[bpos][..(height-1)];
                if suffix == head {
                    // создаём новую ветку
                    let mut new_rem = li.remaining.clone();
                    let board = new_rem.remove(bpos);
                    let mut new_reel = li.reel.clone();
                    new_reel.extend_from_slice(&board[(height-1)..]);
                    let new_inst = ReelInstanse {reel: new_reel,remaining: new_rem,correct: false,};
                    // сохраняем в файл
                    let new_path = tmpdir.path().join(format!("{}-{}.json", iter, bpos));
                    let nf = File::create(&new_path).unwrap();
                    serde_json::to_writer(BufWriter::new(nf), &new_inst).unwrap();
                    paths.push(new_path);
                    pushed = true;
                }
            }
            // если не получилось расширить — ставим маркер отката
            if !pushed {
                li.reel.push(-2);
                let f = File::create(&path).unwrap();
                serde_json::to_writer(BufWriter::new(f), &li).unwrap();
            } else {
                // иначе удаляем «старую» ветку
                fs::remove_file(&path).unwrap();
                paths.remove(idx);
            }
        }
        // дедупликация: оставляем в `paths` только уникальные ReelInstanse
        let mut seen = HashSet::new();
        let mut unique = Vec::new();
        for p in &paths {
            let inst: ReelInstanse = serde_json::from_reader(BufReader::new(File::open(p).unwrap())).unwrap();
            if seen.insert(inst.clone()) {
                unique.push((p.clone(), inst));
            } else {
                // удаляем файл-дубль
                fs::remove_file(p).unwrap();
            }
        }
        // обновляем paths списком тех, что остались
        paths = unique.iter().map(|(p, _)| p.clone()).collect();
        // проверка таймаута
        if can_skip_reel_collect_after_timeout && start.elapsed().unwrap_or(Duration::ZERO) > Duration::from_secs(skip_collect_timeout_sec) {break;}
        pb.inc(1);
    }
    pb.finish_and_clear();
    // читаем оставшиеся ветки в память и возвращаем
    let mut results = Vec::new();
    for p in paths {
        let inst: ReelInstanse = serde_json::from_reader(BufReader::new(File::open(p).unwrap())).unwrap();
        results.push(inst);
    }
    results
}

pub fn _complete_reels(reels: &mut Vec<ReelInstanse>, identical_complete: bool) {
    let pb_main = ProgressBar::new((reels.len()) as u64);
    pb_main.set_prefix("Comleting reels: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    if reels.len() > 0 {
        for pos in 0..reels.len() {
            if identical_complete && reels[pos].remaining.len() > 0 {
                let mut seen: HashSet<Vec<i64>> = HashSet::new(); 
                let mut unique_reels_remain_boards: Vec<Vec<i64>> = Vec::new();
                for remain_board in reels[pos].remaining.iter() {
                    if seen.insert(remain_board.clone()) && remain_board.iter().all(|&x| {x == remain_board[0]}) {
                        unique_reels_remain_boards.push(remain_board.clone());
                    }
                }
                for unique_remain_board in unique_reels_remain_boards.iter() {
                    let mut search_start = 0;
                    for _i in 0..reels[pos].remaining.iter().filter(|&board| board == unique_remain_board).count() {
                        if let Some(insert_pos) = reels[pos].reel[search_start..].windows(unique_remain_board.len()).position(|window| window == unique_remain_board.as_slice()) {
                            let index = search_start + insert_pos;
                            reels[pos].reel.insert(index, unique_remain_board[0]);
                            if let Some(idx) = reels[pos].remaining.iter().position(|board| board == unique_remain_board) {reels[pos].remaining.remove(idx);}
                            search_start = index + 2; 
                            if search_start > reels[pos].reel.len() - unique_remain_board.len() {search_start = 0;}
                        } else {
                            search_start = 0;
                            if let Some(insert_pos) = reels[pos].reel[search_start..].windows(unique_remain_board.len()).position(|window| window == unique_remain_board.as_slice()) {
                                let index = search_start + insert_pos;
                                reels[pos].reel.insert(index, unique_remain_board[0]);
                                if let Some(idx) = reels[pos].remaining.iter().position(|board| board == unique_remain_board) {reels[pos].remaining.remove(idx);}
                                search_start = index + 2; 
                                if search_start > reels[pos].reel.len() - unique_remain_board.len() {search_start = 0;}
                            } else {break;}
                        }
                    }
                }
            }
            pb_main.inc(1);
        }
    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> complete ".to_owned() + &reels.len().to_string() + " reels");
}

pub fn _check_reels(reels: &mut Vec<ReelInstanse>, boards: &Vec<Vec<i64>>, height: usize/*, cut_last_repeating_chars: bool, save_only_one_instance: bool*/) {
    let pb_main = ProgressBar::new((reels.len()) as u64);
    pb_main.set_prefix("Checking reels: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_reels: Vec<ReelInstanse> = reels.clone();
    let mut max_reel_lenght: usize = 0;
    for pos in (0..reels.len()).rev() {
        if max_reel_lenght < reels[pos].reel.len() {max_reel_lenght = reels[pos].reel.len()};
        if reels[pos].reel.len() <= height as usize {reels[pos].correct = true; continue;}
        let mut correct = true;
        for board in boards.iter() {
            let mut search_start = 0;
            let mut finded = false;
            while search_start <= reels[pos].reel.len() - board.len() {
                if let Some(temp_pos) = reels[pos].reel[search_start..].windows(board.len()).position(|window| window == board.as_slice()) {
                    let index = search_start + temp_pos;
                    if l_reels[pos].reel[index] == -1 {search_start = index + 1; continue;} else {l_reels[pos].reel[index] = -1; finded = true; break;}
                } else {break;}
            }
            correct &= finded;
        }
        if l_reels[pos].reel[1..l_reels[pos].reel.len()-height as usize].iter().all(|&x| x == -1) && correct {
            reels[pos].correct = true;
            if reels[pos].reel[reels[pos].reel.len()-1] == -2 {reels[pos].reel.pop();}
            //if cut_last_repeating_chars {for _i in 0..(height-1) {reels[pos].reel.pop();}}
            /*if save_only_one_instance {
                let element = reels.remove(pos);
                let element_checking = l_reels.remove(pos);
                reels.clear();
                l_reels.clear();
                reels.push(element);
                l_reels.push(element_checking);
                break;
            }*/
        } else {
            reels[pos].correct = false;
            if reels[pos].reel.len() < max_reel_lenght {
                reels.remove(pos);
            }
        }
        pb_main.inc(1);
    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> checked ".to_owned() + &reels.len().to_string() + " reels");
}
