use crate::misc::*;
use crate::*;
use colored::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Mutex;

//                                 name    data
type SymbolTable<T> = Lazy<Mutex<HashMap<String, T>>>;
// file      place     value
pub static V_MAP: SymbolTable<(String, Range<usize>, i64)> =
    Lazy::new(|| Mutex::new(HashMap::new()));
// file    place        value
pub static LABEL_MAP: SymbolTable<(String, Range<usize>, usize)> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static MACRO_MAP: SymbolTable<(String, MacroContent)> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static START_LOCATION: Lazy<Mutex<i64>> = Lazy::new(|| Mutex::new(100));

pub static METADATA_STR: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("")));

// - Option<String>: messages about similar entries found (if any)
// - Vec<(String, Range<usize>)>: (file, place) tuples for similar entries
pub fn find_similar_entries(input: &str) -> (Option<String>, Vec<(String, Range<usize>)>) {
    let mut messages = Vec::new();
    let mut results = Vec::new();
    let threshold = 3;

    let v_map = V_MAP.lock().unwrap();
    let label_map = LABEL_MAP.lock().unwrap();
    let macro_map = MACRO_MAP.lock().unwrap();

    let similar_v: Vec<String> = v_map
        .keys()
        .filter(|key| levenshtein(input, key) <= threshold)
        .cloned()
        .collect();
    if !similar_v.is_empty() {
        messages.push(format!(
            "similar constants were found: {}",
            similar_v
                .iter()
                .map(|s| s.magenta().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        for key in similar_v {
            if let Some((file, place, _)) = v_map.get(&key) {
                results.push((file.clone(), place.clone()));
            }
        }
    }

    let similar_labels: Vec<String> = label_map
        .keys()
        .filter(|key| levenshtein(input, key) <= threshold)
        .cloned()
        .collect();
    if !similar_labels.is_empty() {
        messages.push(format!(
            "similar labels were found: {}",
            similar_labels
                .iter()
                .map(|s| s.magenta().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        for key in similar_labels {
            if let Some((file, place, _)) = label_map.get(&key) {
                results.push((file.clone(), place.clone()));
            }
        }
    }

    let similar_macros: Vec<String> = macro_map
        .keys()
        .filter(|key| levenshtein(input, key) <= threshold)
        .cloned()
        .collect();
    if !similar_macros.is_empty() {
        messages.push(format!(
            "similar macros were found: {}",
            similar_macros
                .iter()
                .map(|s| s.magenta().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        for key in similar_macros {
            if let Some((_, mac)) = macro_map.get(&key) {
                results.push((mac.file.to_string(), mac.name.1.clone()));
            }
        }
    }

    if messages.is_empty() {
        (None, results)
    } else {
        (Some(messages.join(", ")), results)
    }
}
use prettytable::{row, Table};
pub fn print_symbol_tables() {
    let v_map = V_MAP.lock().unwrap();
    let l_map = LABEL_MAP.lock().unwrap();
    let m_map = MACRO_MAP.lock().unwrap();
    print_hashmap("Constant map", &v_map);
    print_hashmap("Label map", &l_map);
    print_hashmap("Macro map", &m_map);
}
use crate::tok_print::get_custom_format;
fn print_hashmap<K, V>(name: &str, map: &HashMap<K, V>)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    let mut table = Table::new();
    println!("Contents of {}:", name);
    table.set_format(get_custom_format());
    table.add_row(row!["Key", "Value"]);

    for (key, value) in map {
        let key_str = format!("{:?}", key);
        let value_str = format!("{:?}", value);

        let wrapped_key = wrap_text(&key_str, 80);
        let wrapped_value = wrap_text(&value_str, 80);

        let key_lines: Vec<&str> = wrapped_key.lines().collect();
        let value_lines: Vec<&str> = wrapped_value.lines().collect();

        let max_lines = key_lines.len().max(value_lines.len());

        for i in 0..max_lines {
            let k_line = key_lines.get(i).unwrap_or(&"");
            let v_line = value_lines.get(i).unwrap_or(&"");
            table.add_row(row![k_line, v_line]);
        }
    }

    table.printstd();
}

fn wrap_text(text: &str, max_width: usize) -> String {
    let mut wrapped = String::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
            wrapped.push_str(&current_line);
            wrapped.push('\n');
            current_line.clear();
        }

        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        wrapped.push_str(&current_line);
    }

    wrapped
}
