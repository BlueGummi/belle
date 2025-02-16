use colored::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
pub static LABEL_MAP: Lazy<Mutex<HashMap<String, usize>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static VARIABLE_MAP: Lazy<Mutex<HashMap<String, i32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static MEMORY_COUNTER: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static START_LOCATION: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(100));
pub fn process_start(lines: &[String]) -> Result<(), (usize, String)> {
    let mut start_number: Option<i32> = None;
    let mut start_line = 1;
    for (index, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with(';') {
            continue;
        }

        let line_before_comment = if trimmed_line.contains(';') {
            trimmed_line.split(';').next().unwrap_or(trimmed_line)
        } else {
            trimmed_line
        };

        if line_before_comment.starts_with(".start") {
            start_line = index;
            start_number = line_before_comment.split_whitespace().nth(1).and_then(|s| {
                let stripped = s.strip_prefix('$').unwrap_or(s);
                let stripped = s.strip_prefix('[').unwrap_or(stripped);
                let stripped = if stripped.ends_with(']') {
                    &stripped[0..stripped.len() - 1]
                } else {
                    stripped
                };
                if let Ok(v) = parse_number::<i32>(stripped) {
                    Some(v)
                } else {
                    None
                }
            });
        }
    }

    if let Some(val) = start_number {
        if val > 511 {
            return Err((
                start_line,
                String::from("start location must not exceed 511"),
            ));
        }
    }
    let mut start_location = START_LOCATION
        .lock()
        .map_err(|_| (0, "failed to lock START_LOCATION".to_string()))?;

    *start_location = start_number.unwrap_or(100);

    Ok(())
}
pub fn load_labels(lines: &[String]) -> Result<(), (usize, Option<usize>, String, String)> {
    let mut label_counter = *START_LOCATION.lock().unwrap() as usize;
    let mut label_map = LABEL_MAP.lock().unwrap();
    let mut temp_label_map: HashMap<String, (usize, usize)> = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty()
            || trimmed_line.starts_with(';')
            || trimmed_line.starts_with(".start")
        {
            continue;
        }
        let line_before_comment = if trimmed_line.contains(';') {
            trimmed_line.split(';').next().unwrap_or(trimmed_line)
        } else {
            trimmed_line
        };
        if line_before_comment.trim().ends_with(':') && !line_before_comment.trim().contains(' ') {
            let label_name = line_before_comment
                .trim()
                .trim_end_matches(':')
                .trim()
                .to_string();
            if let Some((_, l)) = temp_label_map.get(&label_name) {
                return Err((
                    index,
                    Some(*l),
                    format!(
                        "duplicate declaration of label \"{}\"",
                        label_name.trim().magenta(),
                    ),
                    format!(
                        "previous label found on line {}",
                        (l + 1).to_string().magenta()
                    ),
                ));
            }
            temp_label_map.insert(label_name.to_string(), (label_counter, index));
            label_map.insert(label_name, label_counter);
            continue;
        }
        if line_before_comment.starts_with(".asciiz") {
            if let Some(start) = trimmed_line.find('\"') {
                if let Some(end) = trimmed_line[start + 1..].find('\"') {
                    label_counter += trimmed_line[start + 1..start + 1 + end].len();
                }
            }
            continue;
        }
        if line_before_comment.starts_with(".word") {
            label_counter += 1;
            continue;
        }

        if line_before_comment.starts_with(".pad") {
            let add = line_before_comment.split_whitespace().nth(1).unwrap_or("0");
            let add = if let Ok(v) = parse_number::<usize>(add) {
                v
            } else {
                return Err((
                    index,
                    None,
                    String::from("could not parse .pad directive"),
                    String::from("valid values require hexadecimal, decimal, or binary strings"),
                ));
            };

            label_counter += add;
            continue;
        }
        if !(line_before_comment.trim().contains('=')
            || line_before_comment.trim().starts_with(".data"))
        {
            label_counter += 1;
        }
    }

    Ok(())
}
pub fn process_variables(lines: &[String]) -> Result<(), (usize, String)> {
    let mut variable_map = VARIABLE_MAP
        .lock()
        .map_err(|_| (0, "failed to lock VARIABLE_MAP".to_string()))?;

    for (index, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with(';') {
            continue;
        }

        let line_before_comment = if trimmed_line.contains(';') {
            trimmed_line.split(';').next().unwrap_or(trimmed_line)
        } else {
            trimmed_line
        };

        if let Some(eq_index) = line_before_comment.find('=') {
            let variable_name = line_before_comment[..eq_index].trim();
            let variable_value = line_before_comment[eq_index + 1..].trim();

            if let Ok(val) = parse_number::<i32>(variable_value) {
                variable_map.insert(variable_name.to_string(), val);
            } else {
                return Err((index, String::from("could not parse variable value")));
            }
        }
    }

    Ok(())
}
pub fn update_memory_counter() -> Result<(), String> {
    let mut counter = MEMORY_COUNTER
        .lock()
        .map_err(|_| "failed to lock MEMORY_COUNTER")?;
    *counter += 1;
    Ok(())
}

use std::num::ParseIntError;
use std::str::FromStr;

trait FromStrRadix: FromStr<Err = ParseIntError> {
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>
    where
        Self: Sized;
}

macro_rules! impl_from_str_radix {
    ($($t:ty),*) => {
        $(impl FromStrRadix for $t {
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                <$t>::from_str_radix(src, radix)
            }
        })*
    };
}

impl_from_str_radix!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

fn parse_number<T: FromStrRadix>(input: &str) -> Result<T, ParseIntError> {
    if let Some(v) = input.strip_prefix("0x") {
        T::from_str_radix(v, 16)
    } else if let Some(v) = input.strip_prefix("0b") {
        T::from_str_radix(v, 2)
    } else {
        input.parse::<T>()
    }
}

#[allow(clippy::needless_range_loop)]
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len_s1 = s1.len();
    let len_s2 = s2.len();

    if len_s1 == 0 {
        return len_s2;
    }
    if len_s2 == 0 {
        return len_s1;
    }

    let mut dp = vec![vec![0; len_s2 + 1]; len_s1 + 1];

    for i in 0..=len_s1 {
        dp[i][0] = i;
    }
    for j in 0..=len_s2 {
        dp[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };

            dp[i + 1][j + 1] = *[dp[i][j + 1] + 1, dp[i + 1][j] + 1, dp[i][j] + cost]
                .iter()
                .min()
                .unwrap();
        }
    }

    dp[len_s1][len_s2]
}

pub fn find_closest_matches(map: &HashMap<String, usize>, query: &str, v: usize) -> Vec<String> {
    let mut matches: Vec<(String, usize)> = map
        .keys()
        .map(|key| (key.clone(), levenshtein_distance(query, key)))
        .filter(|(_, dist)| *dist <= v)
        .collect();

    matches.sort_by_key(|&(_, dist)| dist);

    matches.into_iter().map(|(key, _)| key).collect()
}

pub fn find_closest_matches_i32(map: &HashMap<String, i32>, query: &str) -> Vec<String> {
    let mut matches: Vec<(String, usize)> = map
        .keys()
        .map(|key| (key.clone(), levenshtein_distance(query, key)))
        .filter(|(_, dist)| *dist <= 2)
        .collect();

    matches.sort_by_key(|&(_, dist)| dist);

    matches.into_iter().map(|(key, _)| key).collect()
}
