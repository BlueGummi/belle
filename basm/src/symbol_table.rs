use crate::misc::*;
use crate::*;
use colored::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Mutex;

//                                 name    data
type Table<T> = Lazy<Mutex<HashMap<String, T>>>;
// file      place     value
pub static V_MAP: Table<(String, Range<usize>, i64)> = Lazy::new(|| Mutex::new(HashMap::new()));
// file    place        value
pub static LABEL_MAP: Table<(String, Range<usize>, usize)> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static MACRO_MAP: Table<(String, MacroContent)> = Lazy::new(|| Mutex::new(HashMap::new()));

pub static START_LOCATION: Lazy<Mutex<i64>> = Lazy::new(|| Mutex::new(100));

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
