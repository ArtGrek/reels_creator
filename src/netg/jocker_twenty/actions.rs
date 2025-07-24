use serde_json::Value;
use std::time::{SystemTime, Duration};
use std::collections::HashSet;
//use std::collections::BTreeMap;
//use ordered_float::NotNan;
use indicatif::{ProgressBar, ProgressStyle};
//use std::collections::{HashMap, VecDeque};
//use bitvec::prelude::*;
use crate::netg::jocker_twenty::models::{Categories, Category, Boards, MultiplayedBoardsInstanse, UniqueBoardsInstanse, BoardsInstanse, Board, ReelInstanse, Reel};

pub fn extract(a_transactions: &Vec<Value>, width: usize, _height: usize, a_categories: &mut Categories) {
    let pb_main = ProgressBar::new((a_transactions.len()) as u64);
    pb_main.set_prefix("Extracting data from transactions: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut default_boards: Vec<BoardsInstanse> = vec![];
    let mut multplied_default_boards: Vec<MultiplayedBoardsInstanse> = vec![];
    let mut unique_default_boards: Vec<UniqueBoardsInstanse> = vec![];
    let mut default_reels: Vec<Reel> = vec![];
    for _i in 0..width {
        default_boards.push(BoardsInstanse { instanses: Vec::new() });
        multplied_default_boards.push(MultiplayedBoardsInstanse { instanses: Vec::new() });
        unique_default_boards.push(UniqueBoardsInstanse { count: 0, frequency_average: 0, instanses: Vec::new() });
        default_reels.push(Reel { instanses: Vec::new() });
    }
    a_categories.category.push(Category { boards: Boards { total: default_boards.clone(), filtered: default_boards.clone(), unique: unique_default_boards.clone(), multiplied: multplied_default_boards.clone() }, reels: default_reels.clone(), ..Default::default()} );
    for transaction in a_transactions {
        let board = transaction.get("initialMatrix").and_then(|board| board.as_array()).map(|array_outer| {
            array_outer.iter().filter_map(|array_inner| {
                array_inner.as_array().map(|value| {value.iter().filter_map(|v| {v.as_i64()}).collect::<Vec<i64>>()})
            }).collect::<Vec<Vec<i64>>>()
        }).unwrap_or_default();
        for i in 0..width {
            a_categories.category[0].boards.total[i].instanses.push(board[i].clone().into_iter().map(|x| x as u8).collect::<Vec<u8>>());
        }
        pb_main.inc(1);
    }
    pb_main.finish_with_message(" -> extracted");
}

pub fn _split_by_frequency_average_v2 (width: usize, _height: usize, a_categories: &mut Categories) {
    for i in 0..width {
        a_categories.category[0].boards.total[i].instanses.clear();
        let mut category_num: usize = 0;
        loop {
            calculate_frequency_average(&mut a_categories.category[0].boards.unique[i]);
            let max: u64 = a_categories.category[0].boards.unique[i].instanses.iter().filter(|c| c.count > 0).max_by_key(|c| c.count).unwrap().count;
            if (max as f64 / a_categories.category[0].boards.unique[i].frequency_average as f64).round() <= 5.0 {break;}
            
            category_num += 1;
            if a_categories.category.len() < (category_num + 1) {
                let mut default_boards: Vec<BoardsInstanse> = vec![];
                let mut multplied_default_boards: Vec<MultiplayedBoardsInstanse> = vec![];
                let mut unique_default_boards: Vec<UniqueBoardsInstanse> = vec![];
                let mut default_reels: Vec<Reel> = vec![];
                for _i in 0..width {
                    default_boards.push(BoardsInstanse { instanses: Vec::new() });
                    multplied_default_boards.push(MultiplayedBoardsInstanse { instanses: Vec::new() });
                    unique_default_boards.push(UniqueBoardsInstanse { count: 0, frequency_average: 0, instanses: Vec::new() });
                    default_reels.push(Reel { instanses: Vec::new() });
                }
                let default_category: Category = Category {count: 0,boards: Boards { total: default_boards.clone(), filtered: default_boards.clone(), unique: unique_default_boards.clone(), multiplied: multplied_default_boards.clone() },reels: default_reels.clone()};
                for _i in a_categories.category.len()..(category_num + 1) {a_categories.category.push(default_category.clone());}
            }

            let freq_avg = a_categories.category[0].boards.unique[i].frequency_average as f64;
            let insts_src = &a_categories.category[0].boards.unique[i].instanses;
            let to_push: Vec<_> = insts_src.iter().filter(|inst| (inst.count as f64 / freq_avg).round() < 5.0).cloned().collect();
            let mut unique_count = 0;
            for inst in &to_push {
                unique_count += inst.count;
                a_categories.category[category_num].boards.unique[i].instanses.push(inst.clone());
            }
            a_categories.category[category_num].boards.unique[i].count = unique_count;
            calculate_frequency_average(&mut a_categories.category[category_num].boards.unique[i]);
            {
                let src = &mut a_categories.category[0].boards.unique[i];
                src.instanses.retain(|inst| (inst.count as f64 / freq_avg).round() > 5.0);
                src.count -= unique_count;
            }
            let unique_set: HashSet<Vec<u8>> = a_categories.category[category_num].boards.unique[i].instanses.iter().map(|b| b.board.clone()).collect();
            let to_move: Vec<Vec<u8>> = {
                let src_filtered_ref = &a_categories.category[0].boards.filtered[i].instanses;
                src_filtered_ref.iter().filter(|board_vec| unique_set.contains(*board_vec)).cloned().collect()
            };
            {
                let dst_filtered = &mut a_categories.category[category_num].boards.filtered[i].instanses;
                for board_vec in to_move.iter().cloned() {dst_filtered.push(board_vec);}
            }
            {
                let src_filtered = &mut a_categories.category[0].boards.filtered[i].instanses;
                src_filtered.retain(|board_vec| !unique_set.contains(board_vec));
            }
        }
        if a_categories.category.len() > 1 {
            a_categories.category[1..].reverse();
            for category_num_out in 1..a_categories.category.len() {
                for category_num_in in 0..category_num_out {

                    let freq_avg = a_categories.category[category_num_in].boards.unique[i].frequency_average as i64;
                    let insts_src = &a_categories.category[category_num_in].boards.unique[i].instanses;
                    let to_push: Vec<_> = insts_src.iter().filter(|inst| {

                        let b = (inst.count as i64 - (freq_avg * (inst.count as f64 / freq_avg as f64).round() as i64)) as f64;
                        let a = (b / a_categories.category[category_num_out].boards.unique[i].frequency_average as f64).round();
                        a >= 1.0 && a <= 5.0

                    }).cloned().collect();
                    //let mut unique_count = 0;
                    for inst in &to_push {
                        //unique_count += inst.count;
                        a_categories.category[category_num_out].boards.unique[i].instanses.push(inst.clone());
                    }
                    //a_categories.category[category_num_out].boards.unique[i].count += unique_count;
                    //calculate_frequency_average(&mut a_categories.category[category_num_out].boards.unique[i]);
                    //a_categories.category[category_num_in].boards.unique[i].count -= unique_count;

                }
            }
        }
    }
}

pub fn split_by_frequency_average (width: usize, _height: usize, a_categories: &mut Categories) {
    for i in 0..width {
        a_categories.category[0].boards.total[i].instanses.clear();
        let mut category_num: usize = 0;
        loop {
            calculate_frequency_average(&mut a_categories.category[0].boards.unique[i]);
            let max: u64 = a_categories.category[0].boards.unique[i].instanses.iter().filter(|c| c.count > 0).max_by_key(|c| c.count).unwrap().count;
            if (max as f64 / a_categories.category[0].boards.unique[i].frequency_average as f64).round() <= 5.0 {break;}
            
            category_num += 1;
            if a_categories.category.len() < (category_num + 1) {
                let mut default_boards: Vec<BoardsInstanse> = vec![];
                let mut multplied_default_boards: Vec<MultiplayedBoardsInstanse> = vec![];
                let mut unique_default_boards: Vec<UniqueBoardsInstanse> = vec![];
                let mut default_reels: Vec<Reel> = vec![];
                for _i in 0..width {
                    default_boards.push(BoardsInstanse { instanses: Vec::new() });
                    multplied_default_boards.push(MultiplayedBoardsInstanse { instanses: Vec::new() });
                    unique_default_boards.push(UniqueBoardsInstanse { count: 0, frequency_average: 0, instanses: Vec::new() });
                    default_reels.push(Reel { instanses: Vec::new() });
                }
                let default_category: Category = Category {count: 0,boards: Boards { total: default_boards.clone(), filtered: default_boards.clone(), unique: unique_default_boards.clone(), multiplied: multplied_default_boards.clone() },reels: default_reels.clone()};
                for _i in a_categories.category.len()..(category_num + 1) {a_categories.category.push(default_category.clone());}
            }

            let freq_avg = a_categories.category[0].boards.unique[i].frequency_average as f64;
            let insts_src = &a_categories.category[0].boards.unique[i].instanses;
            let to_push: Vec<_> = insts_src.iter().filter(|inst| (inst.count as f64 / freq_avg).round() < 5.0).cloned().collect();
            let mut unique_count = 0;
            for inst in &to_push {
                unique_count += inst.count;
                a_categories.category[category_num].boards.unique[i].instanses.push(inst.clone());
            }
            a_categories.category[category_num].boards.unique[i].count = unique_count;
            calculate_frequency_average(&mut a_categories.category[category_num].boards.unique[i]);
            {
                let src = &mut a_categories.category[0].boards.unique[i];
                src.instanses.retain(|inst| (inst.count as f64 / freq_avg).round() > 5.0);
                src.count -= unique_count;
            }
            let unique_set: HashSet<Vec<u8>> = a_categories.category[category_num].boards.unique[i].instanses.iter().map(|b| b.board.clone()).collect();
            let to_move: Vec<Vec<u8>> = {
                let src_filtered_ref = &a_categories.category[0].boards.filtered[i].instanses;
                src_filtered_ref.iter().filter(|board_vec| unique_set.contains(*board_vec)).cloned().collect()
            };
            {
                let dst_filtered = &mut a_categories.category[category_num].boards.filtered[i].instanses;
                for board_vec in to_move.iter().cloned() {dst_filtered.push(board_vec);}
            }
            {
                let src_filtered = &mut a_categories.category[0].boards.filtered[i].instanses;
                src_filtered.retain(|board_vec| !unique_set.contains(board_vec));
            }
        }
    }
    if a_categories.category.len() > 1 {
        a_categories.category[1..].reverse();

        for i in 0..width {
            for category_num_out in 1..a_categories.category.len() {
                for category_num_in in 0..category_num_out {
                    let (from_cats, to_cats) = a_categories.category.split_at_mut(category_num_out);
                    let from_cat = &mut from_cats[category_num_in];
                    let to_cat = &mut to_cats[0];
                    let from_unique = &mut from_cat.boards.unique[i];
                    let mut freq_avg_in = from_unique.frequency_average as i64;
                    let to_unique = &mut to_cat.boards.unique[i];
                    let freq_avg_out = to_unique.frequency_average as i64;
                    let from_unique_instanses_len = from_unique.instanses.len();
                    for from_unique_instanses in from_unique.instanses.iter_mut() {
                        let ratio = (from_unique_instanses.count as f64 / freq_avg_in as f64).round() as i64;
                        let b = (from_unique_instanses.count as i64 - (freq_avg_in * ratio)) as f64;
                        let mut a = b / freq_avg_out as f64;
                        eprintln!("col={} from={} to={} board={:?} count={}-({}*{})={} freq_avg={} add={}", i+1, category_num_in, category_num_out, from_unique_instanses.board, 
                        from_unique_instanses.count, freq_avg_in, ratio, b, freq_avg_out, (a >= 0.1 && a <= 5.0));
                        if a >= 0.1 && a <= 5.0 {
                            if a < 1.0 {a = 1.0}
                            let c = (a as i64 * freq_avg_out) as u64;
                            to_unique.instanses.push(Board { count: c, board: from_unique_instanses.board.clone() });
                            to_unique.count += c;
                            if c > from_unique_instanses.count {panic!("c ({}) > from_unique.count ({})", c, from_unique_instanses.count);}
                            from_unique_instanses.count -= c;
                            from_unique.count -= c;
                            freq_avg_in -= (c as f64 / from_unique_instanses_len as f64).round() as i64;
                        eprintln!("avg_in = {} / {} = {}", c, from_unique_instanses_len, freq_avg_in);
                            for _ in 0..c {
                                if let Some(pos) = from_cat.boards.filtered[i].instanses.iter().position(|board| board == &from_unique_instanses.board) {
                                    let board_to_move = from_cat.boards.filtered[i].instanses.remove(pos);
                                    to_cat.boards.filtered[i].instanses.push(board_to_move);
                                } else {break;}
                            }
                        }
                    }
                    calculate_frequency_average(to_unique);
                    calculate_frequency_average(from_unique);
                }
            }
        }

       
    }
}

pub fn delete_boards_with_symbols(a_boards: Vec<Vec<u8>>, symbols: &Vec<i64>) -> Vec<Vec<u8>> {
    let pb_main = ProgressBar::new((a_boards.len()) as u64);
    pb_main.set_prefix("Deleting appering symbols boards: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_boards: Vec<Vec<u8>> = a_boards.clone();
    if l_boards.len() > 0 {
        l_boards.retain(|board| {pb_main.inc(1); !board.iter().any(|symbol| symbols.contains(&(*symbol as i64)))});
    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> remain ".to_owned() + &l_boards.len().to_string() + " boards");
    l_boards
}

pub fn select_unique_boards(a_boards: Vec<Vec<u8>>) -> UniqueBoardsInstanse {
    let pb_main = ProgressBar::new((a_boards.len()) as u64);
    pb_main.set_prefix("Finding unique boards: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_boards: UniqueBoardsInstanse = UniqueBoardsInstanse{ count: 0, frequency_average: 0, instanses: Vec::new() };
    if a_boards.len() > 0 {
        let mut hash_boards: HashSet<Vec<u8>> = HashSet::new();
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

pub fn calculate_frequency_average(boards_unique: &mut UniqueBoardsInstanse) {
        let mut l_counts: Vec<u64> = boards_unique.instanses.iter().map(|x| x.count).collect();
        l_counts.sort();
        let average_min = *l_counts.iter().filter(|&&c| c > 0).min().unwrap();
        let mut average = average_min;
        let mut average_sum = 0;
        let mut average_count = 0;
        l_counts.iter().for_each(|occur_count| {
            if ((*occur_count as f64 / average as f64).round() == 1.0) && ((*occur_count as f64 / average_min as f64).round() <= 2.0) {
                let temp_boards_frequency_average = ((average + occur_count) as f64 / 2.0).round() as u64;
                average = temp_boards_frequency_average;
                average_sum += occur_count;
                average_count += 1;
            }
        });
        boards_unique.frequency_average = (average_sum as f64 / average_count as f64).round() as u64;
}

pub fn _multiply_unique_boards_by_frequency(a_boards_filtered: Vec<Vec<u8>>, boards_unique: &mut UniqueBoardsInstanse, height: usize) -> MultiplayedBoardsInstanse {
    let pb_main = ProgressBar::new((boards_unique.instanses.len()) as u64);
    pb_main.set_prefix("Multipling unique boards: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_boards: MultiplayedBoardsInstanse = MultiplayedBoardsInstanse{ instanses: Vec::new() };
    //match a_boards_filtered.len() {
    match (a_boards_filtered.len() as f64 / boards_unique.instanses.len() as f64).round() as usize {
        100..=usize::MAX => {
            calculate_frequency_average(boards_unique);
            for board_unique_num in 0..boards_unique.instanses.len() {
                let mut l_count = (boards_unique.instanses[board_unique_num].count as f64 / boards_unique.frequency_average as f64).round() as u64;
                if l_count == 0 {l_count = 1;}
                l_boards.instanses.push(Board { count: l_count, board: boards_unique.instanses[board_unique_num].board.clone() });
                pb_main.inc(1);
            }
        },
        1..100 => { 
            l_boards.instanses = boards_unique.instanses.iter().map(|x| Board { count: 1, board: x.board.clone() } ).collect();
            let temp_boards = l_boards.clone();
            loop {
                let mut inserted = false;
                for temp_board in temp_boards.instanses.iter() {
                    let mut unique_left: HashSet<Vec<u8>> = Default::default();
                    let mut unique_right: HashSet<Vec<u8>> = Default::default();
                    let l_left_count = l_boards.instanses.iter().filter(|board| {
                        if board.board[..(height-1) as usize] == temp_board.board[1..] && *board != temp_board && (!temp_board.board[1..].iter().all(|symbol| *symbol == temp_board.board[1]) || temp_board.board.iter().all(|symbol| *symbol == temp_board.board[0]) || board.board.iter().all(|symbol| *symbol == board.board[0])) {
                            unique_left.insert((*board.board.clone()).to_vec())
                        } else {false}
                    }).count();
                    let l_right_count = l_boards.instanses.iter().filter(|board| {
                        if board.board[1..] == temp_board.board[..(height-1) as usize] && *board != temp_board && (!temp_board.board[..(height-1) as usize].iter().all(|symbol| *symbol == temp_board.board[0]) || temp_board.board.iter().all(|symbol| *symbol == temp_board.board[0]) || board.board.iter().all(|symbol| *symbol == board.board[0])) {
                            unique_right.insert((*board.board.clone()).to_vec())
                        } else {false}
                    }).count();
                    let l_count = std::cmp::max(l_left_count, l_right_count);
                    let l_exist_count = l_boards.instanses.iter().filter(|board| {*board == temp_board}).count();
                    if l_count > l_exist_count {
                        inserted = true;
                        for _i in 0..l_count-l_exist_count {
                            l_boards.instanses.push(temp_board.clone());
                        }
                    }
                    pb_main.inc(1);
                } 
                if !inserted {break;}     
            }
        },
        _ => {l_boards.instanses = boards_unique.instanses.iter().map(|x| {pb_main.inc(1); Board { count: 1, board: x.board.clone()}}).collect();}
    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> result ".to_owned() + &l_boards.len().to_string() + " boards");
    l_boards
}

/*pub fn _collect_reels(a_reel_num: String, a_boards: &Vec<Board>, height: usize, can_skip_reel_collect_after_timeout: bool, skip_collect_timeout_sec: u64) -> Vec<ReelInstanse> {
    let l_boards_len = a_boards.len();
    let pb_main = ProgressBar::new(l_boards_len as u64);
    pb_main.set_prefix(format!("Collecting reels {}: ", a_reel_num)); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_reels: Vec<ReelInstanse> = Vec::new();
    if l_boards_len > 0 {
        let mut temp_boards: Vec<Vec<i64>> = a_boards.iter().map(|x| x.board.clone()).collect();
        let temp_board: Vec<i64> = temp_boards.remove(0);
        let temp_reel: Vec<i64> = temp_board;
        l_reels.push(ReelInstanse {reel: temp_reel, remaining: temp_boards, correct: false});
        let collect_start_time = SystemTime::now();

        for _i in 0..l_boards_len-1 {
            if l_reels.is_empty() {break;}
            let mut stop_pos = l_reels.len()-1;
            let start_pos = 0;
            loop {
                if l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-1] != -2 {
                    let mut pushed = false;
                    for board_pos in (0..l_reels[stop_pos].remaining.len()).rev() {
                        let same_count = l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - height as usize ..].iter().rev().take_while(|&&x| {x == l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - 1]}).count();
                        let next_equal = l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-1] == l_reels[stop_pos].remaining[board_pos][l_reels[stop_pos].remaining[board_pos].len()-1];
                        if l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-(height-1) as usize..] == l_reels[stop_pos].remaining[board_pos][..(height-1) as usize] 
                        && (false || !((2..height as usize).contains(&same_count) && !next_equal))
                        && !(same_count == height as usize && next_equal)
                        {
                            let mut temp_boards = l_reels[stop_pos].remaining.clone();
                            temp_boards.remove(board_pos);
                            let mut temp_reel = l_reels[stop_pos].reel.clone();
                            let new_board = l_reels[stop_pos].remaining[board_pos][(height-1) as usize..].to_vec().clone();
                            temp_reel.extend_from_slice(&new_board);
                            l_reels.push(ReelInstanse {reel: temp_reel.clone(), remaining: temp_boards.clone(), correct: false});
                            pushed = true;
                        }
                        else {
                            let same_count = l_reels[stop_pos].reel[..height as usize].iter().rev().take_while(|&&x| {x == l_reels[stop_pos].reel[0]}).count();
                            let next_equal = l_reels[stop_pos].reel[0] == l_reels[stop_pos].remaining[board_pos][0];
                            if l_reels[stop_pos].reel[..(height - 1) as usize]== l_reels[stop_pos].remaining[board_pos][l_reels[stop_pos].remaining[board_pos].len() - (height - 1) as usize..]
                            && (false || !((2..height as usize).contains(&same_count) && !next_equal))
                            && !(same_count == height as usize && next_equal)
                            {
                                let mut temp_boards = l_reels[stop_pos].remaining.clone();
                                temp_boards.remove(board_pos);
                                let mut temp_reel = l_reels[stop_pos].reel.clone();
                                let new_board = l_reels[stop_pos].remaining[board_pos][..1].to_vec().clone();
                                temp_reel.splice(0..0,new_board);
                                l_reels.push(ReelInstanse {reel: temp_reel.clone(), remaining: temp_boards.clone(), correct: false});
                                pushed = true;
                            }
                        }
                    }
                    if pushed {l_reels.remove(stop_pos);} else {l_reels[stop_pos].reel.push(-2);}
                } //else {if l_reels[stop_pos].reel.len() < i {l_reels.remove(stop_pos);}}
                if start_pos == stop_pos {break;} stop_pos -= 1; 
            }
            let mut seen: HashSet<ReelInstanse> = HashSet::new(); let mut unique_reels: Vec<ReelInstanse> = Vec::new();
            for l_reel in l_reels.iter() {if seen.insert(l_reel.clone()) {unique_reels.push(l_reel.clone());} }
            l_reels = unique_reels.clone();
            
            if can_skip_reel_collect_after_timeout {if SystemTime::now() > collect_start_time + Duration::from_secs(skip_collect_timeout_sec as u64) {break;}}
            pb_main.inc(1);
        }

    }
    pb_main.finish_and_clear();
    //pb_main.finish_with_message(" -> collect ".to_owned() + &l_reels.len().to_string() + " reels");
    l_reels
}*/

pub fn _collect_reels(a_reel_num: String, a_boards: &Vec<Board>, height: usize, can_skip_reel_collect_after_timeout: bool, skip_collect_timeout_sec: u64) -> Vec<ReelInstanse> {
    let l_boards_len: u64 = a_boards.iter().map(|board| {board.count}).sum();
    let pb_main = ProgressBar::new(l_boards_len as u64);
    pb_main.set_prefix(format!("Collecting reels {}: ", a_reel_num)); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_reels: Vec<ReelInstanse> = Vec::new();
    if l_boards_len > 0 {
        let mut temp_boards: Vec<Board> = a_boards.clone();
        temp_boards[0].count -= 1;
        let temp_reel: Vec<u8> =  temp_boards[0].board.clone();
        if temp_boards[0].count < 1 {temp_boards.remove(0);}
        l_reels.push(ReelInstanse {reel: temp_reel, remaining: temp_boards, correct: false});
        let collect_start_time = SystemTime::now();
        //let mut max_reel_lenght: usize = 3;
        for _i in 0..(l_boards_len-1) as usize {
            if l_reels.is_empty() {break;}
            let mut stop_pos = l_reels.len()-1;
            let start_pos = 0;
            loop {
                let mut reels_to_add: Vec<ReelInstanse> = Vec::new();
                let current_reels = &mut l_reels[stop_pos];
                let current_reel = &mut current_reels.reel;
                if current_reel[current_reel.len()-1] != u8::MAX-2 {
                    let mut pushed = false;
                    let current_boards = &current_reels.remaining;
                    for board_pos in (0..current_boards.len()).rev() {
                        let current_board = &current_boards[board_pos];
                        let current_reel_last = current_reel[current_reel.len()-1];
                        let current_reel_first = current_reel[0];
                        let current_board_last = current_board.board[current_board.board.len()-1];
                        let current_board_first = current_board.board[0];
                        let next_equal_r = current_reel_last == current_board_last;
                        let next_equal_l = current_reel_first == current_board_first;
                        let same_count_r = current_reel[current_reel.len() - height as usize ..].iter().rev().take_while(|&&x| {x == current_reel_last}).count();
                        let same_count_l = current_reel[..height as usize].iter().take_while(|&&x| {x == current_reel_first}).count();
                        if current_reel[current_reel.len()-(height-1) as usize..] == current_board.board[..(height-1) as usize] 
                        && (false || !((2..height as usize).contains(&same_count_r) && !next_equal_r))
                        && !(same_count_r == height as usize && next_equal_r)
                        {
                            let mut temp_boards = current_boards.clone();
                            temp_boards[board_pos].count -= 1;
                            let mut temp_reel = current_reel.clone();
                            let new_board = current_board.board[(height-1) as usize..].to_vec().clone();
                            temp_reel.extend_from_slice(&new_board);
                            //if temp_reel.len() > max_reel_lenght {max_reel_lenght = temp_reel.len();}
                            if temp_boards[board_pos].count < 1 {temp_boards.remove(board_pos);}
                            reels_to_add.push(ReelInstanse {reel: temp_reel, remaining: temp_boards, correct: false});
                            pushed = true;
                        }
                        else {
                            if current_reel[..(height - 1) as usize] == current_board.board[current_board.board.len() - (height - 1) as usize..] 
                            && (false || !((2..height as usize).contains(&same_count_l) && !next_equal_l))
                            && !(same_count_l == height as usize && next_equal_l)
                            {
                                let mut temp_boards = current_boards.clone();
                                temp_boards[board_pos].count -= 1;
                                let mut temp_reel = current_reel.clone();
                                let new_board = current_board.board[..1].to_vec().clone();
                                temp_reel.splice(0..0,new_board);
                                //if temp_reel.len() > max_reel_lenght {max_reel_lenght = temp_reel.len();}
                                if temp_boards[board_pos].count < 1 {temp_boards.remove(board_pos);}
                                reels_to_add.push(ReelInstanse {reel: temp_reel, remaining: temp_boards, correct: false});
                                pushed = true;
                            }
                        }
                    }
                    if pushed {
                        l_reels.remove(stop_pos);
                        l_reels.extend(reels_to_add);
                    } else {
                        l_reels[stop_pos].reel.push(u8::MAX-2);
                    }
                } //else {if (l_reels[stop_pos].reel.len()-3) < i && l_reels[stop_pos].reel.len() < max_reel_lenght {l_reels.remove(stop_pos);}}
                if start_pos == stop_pos {break;} stop_pos -= 1; 
            }
            let mut seen: HashSet<ReelInstanse> = HashSet::new(); let mut unique_reels: Vec<ReelInstanse> = Vec::new();
            for l_reel in l_reels.iter() {if seen.insert(l_reel.clone()) {unique_reels.push(l_reel.clone());} }
            l_reels = unique_reels.clone();
            
            if can_skip_reel_collect_after_timeout {if SystemTime::now() > collect_start_time + Duration::from_secs(skip_collect_timeout_sec as u64) {break;}}
            pb_main.inc(1);
        }
    }
    pb_main.finish_and_clear();
    l_reels
}

pub fn _collect_reels_v2(a_reel_num: String, a_boards: &Vec<Board>, height: usize, can_skip_reel_collect_after_timeout: bool, skip_collect_timeout_sec: u64) -> Vec<ReelInstanse> {
    let l_boards_len: u64 = a_boards.iter().map(|board| {board.count}).sum();
    let pb_main = ProgressBar::new(l_boards_len as u64);
    pb_main.set_prefix(format!("Collecting reels {}: ", a_reel_num)); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_reels: Vec<ReelInstanse> = Vec::new();
    if l_boards_len > 0 {
        let mut temp_boards: Vec<Board> = a_boards.clone();
        temp_boards[0].count -= 1;
        let temp_reel: Vec<u8> =  temp_boards[0].board.clone();
        if temp_boards[0].count < 1 {temp_boards.remove(0);}
        l_reels.push(ReelInstanse {reel: temp_reel, remaining: temp_boards, correct: false});
        let collect_start_time = SystemTime::now();
        for _i in 0..(l_boards_len-1) as usize {
            if l_reels.is_empty() {break;}
            let mut stop_pos = l_reels.len()-1;
            let start_pos = 0;
            loop {
                let current_reels = &mut l_reels[stop_pos];
                let current_reel = &mut current_reels.reel;
                if current_reel[current_reel.len()-1] != u8::MAX-2 {
                    let mut pushed = false;
                    let current_boards = &mut current_reels.remaining;
                    for board_pos in (0..current_boards.len()).rev() {
                        let current_board = &mut current_boards[board_pos];
                        let current_reel_last = current_reel[current_reel.len()-1];
                        let current_reel_first = current_reel[0];
                        let current_board_last = current_board.board[current_board.board.len()-1];
                        let current_board_first = current_board.board[0];
                        let next_equal_r = current_reel_last == current_board_last;
                        let next_equal_l = current_reel_first == current_board_first;
                        let same_count_r = current_reel[current_reel.len() - height as usize ..].iter().rev().take_while(|&&x| {x == current_reel_last}).count();
                        let same_count_l = current_reel[..height as usize].iter().take_while(|&&x| {x == current_reel_first}).count();
/*println!("{:?}", current_reel); 
println!("{:?}", current_board); 
println!("{:?} <{:?}> {:?}", &current_reel[current_reel.len()-(height-1) as usize..], current_reel[current_reel.len()-(height-1) as usize..] == current_board.board[..(height-1) as usize], &current_board.board[..(height-1) as usize]); 
println!("current_reel_right_slice: {:?}, current_reel_last: {:?}", &current_reel[current_reel.len() - height as usize ..], current_reel_last);
println!("same_count_r: {:?}, next_equal_r: {:?}", same_count_r, next_equal_r);
println!("{:?}", !((2..height as usize).contains(&same_count_r) && !next_equal_r)); 
println!("{:?}", !(same_count_r == height as usize && next_equal_r)); 
println!("{:?} <{:?}> {:?}", &current_reel[..(height - 1) as usize], current_reel[..(height - 1) as usize] == current_board.board[current_board.board.len() - (height - 1) as usize..], &current_board.board[current_board.board.len() - (height - 1) as usize..]);
println!("current_reel_left_slice: {:?}, current_reel_first: {:?}", &current_reel[..height as usize], current_reel_first);
println!("same_count_l: {:?}, next_equal_l: {:?}", same_count_l, next_equal_l);
println!("{:?}", !((2..height as usize).contains(&same_count_l) && !next_equal_l)); 
println!("{:?}", !(same_count_l == height as usize && next_equal_l));  
let _ = std::io::stdin().read_line(&mut String::new());*/
                        if current_reel[current_reel.len()-(height-1) as usize..] == current_board.board[..(height-1) as usize] 
                        && (false || !((2..height as usize).contains(&same_count_r) && !next_equal_r))
                        && !(same_count_r == height as usize && next_equal_r)
                        {
                            let new_board = current_board.board[(height-1) as usize..].to_vec().clone();
                            current_reel.extend_from_slice(&new_board);
                            current_board.count -= 1;
                            if current_board.count < 1 {current_boards.remove(board_pos);}
                            pushed = true;
                        }
                        else {
                            if current_reel[..(height - 1) as usize] == current_board.board[current_board.board.len() - (height - 1) as usize..] 
                            && (false || !((2..height as usize).contains(&same_count_l) && !next_equal_l))
                            && !(same_count_l == height as usize && next_equal_l)
                            {
                                let new_board = current_board.board[..1].to_vec().clone();
                                current_reel.splice(0..0,new_board);
                                current_board.count -= 1;
                                if current_board.count < 1 {current_boards.remove(board_pos);}
                                pushed = true;
                            }
                        }
                    }
                    if pushed {
                        //l_reels.remove(stop_pos);
                    } else {
                        current_reel.push(u8::MAX-2);
                        if current_boards.len() > 0 {
                            let mut temp_boards: Vec<Board> = current_boards.clone();
                            temp_boards[0].count -= 1;
                            let temp_reel: Vec<u8> =  temp_boards[0].board.clone();
                            if temp_boards[0].count < 1 {temp_boards.remove(0);}
                            l_reels.push(ReelInstanse {reel: temp_reel, remaining: temp_boards, correct: false});
                        }
                    }
                }
                if start_pos == stop_pos {break;} stop_pos -= 1; 
            }
            
            if can_skip_reel_collect_after_timeout {if SystemTime::now() > collect_start_time + Duration::from_secs(skip_collect_timeout_sec as u64) {break;}}
            pb_main.inc(1);
        }
    }
    pb_main.finish_and_clear();
/*print!("\x1B[2J\x1B[1;1H"); 
println!("{:?}", l_reels); 
let _ = std::io::stdin().read_line(&mut String::new());*/
    l_reels
}

/*pub fn _collect_reels_v2(a_reel_num: String, a_boards: &Vec<Board>, height: usize, can_skip_reel_collect_after_timeout: bool, skip_collect_timeout_sec: u64) -> Vec<ReelInstanse> {
    let pb_main = ProgressBar::new((a_boards.len()) as u64);
    pb_main.set_prefix(format!("Collecting reels {}: ", a_reel_num)); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    let mut l_reels: Vec<ReelInstanse> = Vec::new();
    if a_boards.len() > 0 {
        let mut temp_boards: Vec<Board> = a_boards.clone();
        temp_boards[0].count -= 1;
        l_reels.push(ReelInstanse {reel: temp_boards[0].board.clone(), remaining: temp_boards, correct: false});
        let collect_start_time = SystemTime::now();
        for _i in 0..a_boards.len()-1 {

            if l_reels.len() > 0 {
                let mut stop_pos = l_reels.len()-1;
                let start_pos = 0;
                loop {
                    if l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-1] != u8::MAX-2 {
                        let mut pushed = false;
                        for board_pos in (0..l_reels[stop_pos].remaining.len()).rev() {
                            if l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-(height-1) as usize..] == l_reels[stop_pos].remaining[board_pos].board[..(height-1) as usize] 
                            && (false || !(
                                (2..height as usize).contains(&l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - height as usize ..].iter().rev().take_while(|&&x| {x == l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - 1]}).count())
                                && l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-1] != l_reels[stop_pos].remaining[board_pos].board[l_reels[stop_pos].remaining[board_pos].board.len()-1]
                            ))
                            && !(
                                l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - height as usize ..].iter().rev().filter(|&&x| {x == l_reels[stop_pos].reel[l_reels[stop_pos].reel.len() - 1]}).count() == height as usize
                                && l_reels[stop_pos].reel[l_reels[stop_pos].reel.len()-1] == l_reels[stop_pos].remaining[board_pos].board[l_reels[stop_pos].remaining[board_pos].board.len()-1]
                            )
                            {
                                let new_board = l_reels[stop_pos].remaining[board_pos].board[(height-1) as usize..].to_vec().clone();
                                l_reels[stop_pos].reel.extend_from_slice(&new_board);
                                l_reels[stop_pos].remaining.remove(board_pos);
                                pushed = true;
                            }
                            else if l_reels[stop_pos].reel[..(height - 1) as usize]== l_reels[stop_pos].remaining[board_pos].board[l_reels[stop_pos].remaining[board_pos].board.len() - (height - 1) as usize..]
                            && (false || !(
                                (2..height as usize).contains(&l_reels[stop_pos].reel[..height as usize].iter().take_while(|&&x| x == l_reels[stop_pos].reel[0]).count())
                                && l_reels[stop_pos].reel[0] != l_reels[stop_pos].remaining[board_pos].board[0]
                            ))
                            && !(
                                l_reels[stop_pos].reel[..height as usize].iter().filter(|&&x| x == l_reels[stop_pos].reel[0]).count() == height as usize
                                && l_reels[stop_pos].reel[0] == l_reels[stop_pos].remaining[board_pos].board[0]
                            )
                            {
                                let new_board = l_reels[stop_pos].remaining[board_pos].board[..1].to_vec().clone();
                                l_reels[stop_pos].reel.splice(0..0, new_board);
                                l_reels[stop_pos].remaining.remove(board_pos);
                                pushed = true;
                            }
                        }
                        if pushed {
                            //l_reels.remove(stop_pos);
                        } else {
                            l_reels[stop_pos].reel.push(u8::MAX-2);
                            if l_reels[stop_pos].remaining.len() > 0 {
                                let mut temp_boards: Vec<Board> = l_reels[stop_pos].remaining.clone();
                                temp_boards[0].count -= 1;
                                l_reels.push(ReelInstanse {reel: temp_boards[0].board.clone(), remaining: temp_boards, correct: false});
                            }
                        }
                    } //else {if l_reels[stop_pos].reel.len() < i {l_reels.remove(stop_pos);}}
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
}*/

pub fn complete_reels(reels: &mut Vec<ReelInstanse>, identical_complete: bool) {
    let pb_main = ProgressBar::new((reels.len()) as u64);
    pb_main.set_prefix("Comleting reels: "); 
    pb_main.set_style(ProgressStyle::default_bar().template("{prefix} [{bar:100.cyan/blue}] {pos}/{len} {msg}").expect("ProgressBar template error"),);
    if reels.len() > 0 {
        for pos in 0..reels.len() {
            if identical_complete && reels[pos].remaining.len() > 0 {
                let mut seen: HashSet<Vec<u8>> = HashSet::new(); 
                let mut unique_reels_remain_boards: Vec<Vec<u8>> = Vec::new();
                for remain_board in reels[pos].remaining.iter() {
                    if seen.insert(remain_board.board.clone()) && remain_board.board.iter().all(|&x| {x == remain_board.board[0]}) {
                        unique_reels_remain_boards.push(remain_board.board.clone());
                    }
                }
                for unique_remain_board in unique_reels_remain_boards.iter() {
                    let mut search_start = 0;
                    for _i in 0..reels[pos].remaining.iter().filter(|&board| board.board == *unique_remain_board).count() {
                        if let Some(insert_pos) = reels[pos].reel[search_start..].windows(unique_remain_board.len()).position(|window| window == unique_remain_board.as_slice()) {
                            let index = search_start + insert_pos;
                            reels[pos].reel.insert(index, unique_remain_board[0]);
                            if let Some(idx) = reels[pos].remaining.iter().position(|board| board.board == *unique_remain_board) {reels[pos].remaining.remove(idx);}
                            search_start = index + 2; 
                            if search_start > reels[pos].reel.len() - unique_remain_board.len() {search_start = 0;}
                        } else {
                            search_start = 0;
                            if let Some(insert_pos) = reels[pos].reel[search_start..].windows(unique_remain_board.len()).position(|window| window == unique_remain_board.as_slice()) {
                                let index = search_start + insert_pos;
                                reels[pos].reel.insert(index, unique_remain_board[0]);
                                if let Some(idx) = reels[pos].remaining.iter().position(|board| board.board == *unique_remain_board) {reels[pos].remaining.remove(idx);}
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

pub fn check_reels(reels: &mut Vec<ReelInstanse>, boards: &Vec<Board>, height: usize/*, cut_last_repeating_chars: bool, save_only_one_instance: bool*/) {
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
            while search_start <= reels[pos].reel.len() - board.board.len() {
                if let Some(temp_pos) = reels[pos].reel[search_start..].windows(board.board.len()).position(|window| window == board.board.as_slice()) {
                    let index = search_start + temp_pos;
                    if l_reels[pos].reel[index] == u8::MAX-1 {search_start = index + 1; continue;} else {l_reels[pos].reel[index] = u8::MAX-1; finded = true; break;}
                } else {break;}
            }
            correct &= finded;
        }
        if l_reels[pos].reel[1..l_reels[pos].reel.len()-height as usize].iter().all(|&x| x == u8::MAX-1) && correct {
            reels[pos].correct = true;
            if reels[pos].reel[reels[pos].reel.len()-1] == u8::MAX-2 {reels[pos].reel.pop();}
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
