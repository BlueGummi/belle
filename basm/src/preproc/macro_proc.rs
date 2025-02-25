use crate::*;
use colored::*;
use std::collections::HashMap;
use std::ops::Range;

pub fn process_macros(toks: &mut Vec<(String, TokenKind, Range<usize>)>, error_count: &mut i32) {
    let mut mac_locs = Vec::new();
    for (index, (_fname, element, _)) in toks.iter().enumerate() {
        if let Macro(data) = element {
            let mut mac_map = MACRO_MAP.lock().unwrap();
            mac_map.insert(
                data.name.0.to_string(),
                (data.file.to_string(), data.clone()),
            );
            mac_locs.push(index);
        }
    }

    for element in mac_locs.iter().rev() {
        toks.remove(*element);
    }

    let mut mac_call_data = Vec::new();
    let mut in_call = false;
    let mut curr_mac = None;

    let mut expanded_loc_map: HashMap<usize, Vec<(TokenKind, Range<usize>)>> = HashMap::new();
    let mut expanded_indices = Vec::new();

    let mac_map = MACRO_MAP.lock().unwrap();
    let mut counter = 0;
    for (fname, element, span) in toks.iter() {
        counter += 1;
        if let MacroCall(call) = element {
            in_call = true;
            mac_call_data = Vec::new();
            mac_call_data.push((MacroCall(call.to_string()), span.clone()));
            if let Some(v) = mac_map.get(call) {
                curr_mac = Some(v);
            } else {
                std::mem::drop(mac_map);
                let info = find_similar_entries(call);
                let info = if let (Some(s), _) = info {
                    Some(format!("{} {s}", "╮".bright_red()))
                } else {
                    None
                };
                handle_core_error(fname, span, error_count, "cannot find macro", info);
                break;
            }
            continue;
        }
        if let RightParen = element {
            in_call = false;
            if let Some((_, m)) = curr_mac {
                match m.is_valid(fname.to_string(), read_file(fname), mac_call_data.clone()) {
                    Ok(v) => {
                        expanded_loc_map.insert(counter, v.clone());
                        expanded_indices.push(counter);
                    }
                    Err(errors) => {
                        for e in errors {
                            println!("{e}\n");
                            *error_count += 1;
                        }
                    }
                }
            }
            continue;
        }
        if in_call {
            mac_call_data.push((element.clone(), span.clone()));
        }
    }

    let size = toks.len();
    for i in 0..size {
        if expanded_indices.contains(&i) {
            let expanded = expanded_loc_map.get(&i).unwrap();
            for element in expanded.iter().rev() {
                let (x, y) = element;
                toks.insert(
                    i,
                    (
                        String::from("NULL: EXPANDED FROM MACRO"),
                        x.clone(),
                        y.clone(),
                    ),
                );
            }
        }
    }
    use crate::TokenKind::*;

    let mut new_tokens = Vec::new();
    let mut tokerator = toks.clone().into_iter();
    while let Some((f, v, s)) = tokerator.next() {
        match v {
            MacroCall(_) => {
                for (_, val, _) in tokerator.by_ref() {
                    if val == RightParen {
                        break;
                    }
                }
            }
            _ => new_tokens.push((f, v, s)),
        }
    }
    print_errc!(*error_count);
    *toks = new_tokens;
}
