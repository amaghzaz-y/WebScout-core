use itertools::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[derive(Serialize, Deserialize)]
pub struct PairSet {
    map: HashMap<[u8; 2], HashSet<String>>,
}

impl PairSet {
    pub fn new() -> PairSet {
        PairSet {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, value: String) {
        let mut token: String = value
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        token.make_ascii_lowercase();
        if token.len() > 1 {
            let prefix: [u8; 2] = token
                .chars()
                .take(2)
                .map(|c| c as u8)
                .collect_vec()
                .try_into()
                .expect("slice with incorrect length");
            self.map
                .entry(prefix)
                .or_insert(HashSet::default())
                .insert(token);
        }
    }
    pub fn get(&self, value: &String) -> Option<String> {
        let mut result: Option<String> = None;
        let mut token: String = value
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        token.make_ascii_lowercase();
        if value.len() > 1 {
            let prefix: [u8; 2] = token
                .chars()
                .take(2)
                .map(|c| c as u8)
                .collect_vec()
                .try_into()
                .expect("slice with incorrect length");
            let root = self.map.get(&prefix);
            if root.is_some() {
                let s = root.unwrap().get(value);
                if s.is_some() {
                    result = Some(s.unwrap().to_owned());
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        return result;
    }
    pub fn children(&self, value: &String) {
        let mut result: Option<HashSet<String>> = None;
        let prefix: [u8; 2] = value
            .chars()
            .take(2)
            .map(|mut c| {
                c.make_ascii_lowercase();
                return c as u8;
            })
            .collect_vec()
            .try_into()
            .expect("slice with incorrect length");
        let children = self.map.get(&prefix);
        if children.is_some() {
            result = Some(children.unwrap().to_owned())
        }
    }
    pub fn delete(&mut self, value: &String) {
        let prefix: [u8; 2] = value
            .chars()
            .take(2)
            .map(|mut c| {
                c.make_ascii_lowercase();
                return c as u8;
            })
            .collect_vec()
            .try_into()
            .expect("slice with incorrect length");
        self.map.entry(prefix).or_default().remove(value);
    }
}
