use crate::types::Document;
use std::collections::{HashMap, HashSet};
pub fn parse_string(mut text: String) -> HashMap<String, HashSet<usize>> {
    let mut map: HashMap<String, HashSet<usize>> = HashMap::default();
    let mut chars: Vec<u8> = vec![];
    let mut count: usize = 0;
    text.push('/'); // to mark the end of document
    for char in text.as_bytes() {
        if char.is_ascii_alphanumeric() {
            chars.push(*char);
        } else {
            if chars.len() > 1 {
                let mut word = String::from_utf8(chars.to_owned()).unwrap();
                word.make_ascii_lowercase();
                map.entry(word)
                    .or_insert(HashSet::from([count]))
                    .insert(count);
            }
            count += 1;
            chars.clear();
        }
    }
    return map;
}
