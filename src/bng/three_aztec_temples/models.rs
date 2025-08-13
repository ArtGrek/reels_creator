use serde::{Deserialize, Serialize};
use serde::ser::{SerializeMap, Serializer};
use serde_json::Value;
use ordered_float::NotNan;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Row {
    pub count: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Col {
    pub count: i64,
    pub rows: Vec<Row>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Symbol {
    pub id: i64,
    pub count: i64,
    #[serde(serialize_with="serialize_map_as_string_keys")]
    pub values: BTreeMap<Multi, i64>,
    #[serde(serialize_with="serialize_map_as_string_keys")]
    pub multiplayers: BTreeMap<i64, i64>,
    pub cols: Vec<Col>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct ByBonusLenght {
    pub respins: i64,
    pub emerged: i64,
    #[serde(serialize_with="serialize_map_as_string_keys")]
    pub combo_symbols: BTreeMap<Vec<i64>, i64>,
    pub symbols: Vec<Symbol>,
    pub mystery_symbols: Vec<Symbol>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct ByMechanic {
    pub id: Vec<i64>,
    pub appearances: i64,
    pub inits: i64,
    pub respins: i64,
    pub emerged: i64,
    pub amount: i64,
    pub reappearances: i64,
    #[serde(serialize_with="serialize_map_as_string_keys")]
    pub combo_symbols: BTreeMap<Vec<i64>, i64>,
    pub reinits: i64,
    pub symbols: Vec<Symbol>,
    pub mystery_symbols: Vec<Symbol>,
    pub by_bonus_lenghts: Vec<ByBonusLenght>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Bonus {
    pub id: String,
    pub appearances: i64,
    pub inits: i64,
    pub respins: i64,
    pub emerged: i64,
    pub amount: i64,
    pub reappearances: i64,
    #[serde(serialize_with="serialize_map_as_string_keys")]
    pub combo_symbols: BTreeMap<Vec<i64>, i64>,
    pub reinits: i64,
    pub symbols: Vec<Symbol>,
    pub mystery_symbols: Vec<Symbol>,
    pub by_bonus_lenghts: Vec<ByBonusLenght>,
    pub by_mechanics: Vec<ByMechanic>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Bet {
    pub count: i64,
    pub amount: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct ByLenght {
    pub length: i64,
    pub count: i64,
    pub amount: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct ByLine {
    pub id: i64,
    pub count: i64,
    pub amount: i64,
    pub by_lengths: Vec<ByLenght>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct BySymbol {
    pub id: i64,
    pub count: i64,
    pub amount: i64,
    pub by_lengths: Vec<ByLenght>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Win {
    pub count: i64,
    pub amount: i64,
    pub by_lines: Vec<ByLine>,
    pub by_symbols: Vec<BySymbol>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Spins {
    pub id: String,
    pub count: i64,
    pub bet: Bet,
    pub win: Win,
    pub symbols: Vec<Symbol>,
    pub reels: Vec<Reel>,
    pub boards: Boards,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Mode {
    pub count: i64,
    pub spins: HashMap<String, Spins>,
    pub bonus: HashMap<String, Bonus>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Game {
    pub base: Mode,
    pub buy_1: Mode,
    pub buy_2: Mode,
    pub settings: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum Multi {
    Float(NotNan<f64>),
    Int(i64),
    String(String),
}

impl Multi {
    pub fn as_string(&self) -> String {
        match self {
            Multi::Int(i)    => i.to_string(),
            Multi::Float(f)  => f.to_string(),
            Multi::String(s) => s.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Board {
    pub count: i64,
    pub board: Vec<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct UniqueBoardsInstanse {
    pub count: i64,
    pub frequency_average: i64,
    pub instanses: Vec<Board>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct BoardsInstanse {
    pub instanses: Vec<Vec<i64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Boards {
    pub total: Vec<BoardsInstanse>,
    pub filtered: Vec<BoardsInstanse>,
    pub unique: Vec<UniqueBoardsInstanse>,
    pub multiplied: Vec<BoardsInstanse>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct ReelInstanse {
    pub reel: Vec<i64>,
    pub remaining: Vec<Vec<i64>>,
    pub correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Reel {
    pub instanses: Vec<ReelInstanse>,
}

/// Универсальный сериализатор для любых BTreeMap<K, V>,
/// где K: Serialize + Clone
/// Мы берём JSON-представление ключа через serde_json::to_string,
/// затем снимаем внешние кавычки, если они есть.
pub fn serialize_map_as_string_keys<K, V, S>(
    map: &BTreeMap<K, V>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    K: Serialize + Clone,
    V: Serialize,
    S: Serializer,
{
    let mut m = serializer.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        // 1) Сериализуем k в JSON-строку
        let mut key_str = serde_json::to_string(k)
            .map_err(serde::ser::Error::custom)?;
        // 2) Если это было строкой (обёрнуто в кавычки), убираем их
        if key_str.len() >= 2 && key_str.starts_with('"') && key_str.ends_with('"') {
            key_str = key_str[1..key_str.len()-1].to_string();
        }
        // 3) Добавляем в результат
        m.serialize_entry(&key_str, v)?;
    }
    m.end()
}