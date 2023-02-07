use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub id: usize,
    pub lang: String,
    pub index: HashMap<String, HashSet<usize>>, // words with it's current position on document
}
#[derive(Serialize, Deserialize)]
pub struct Lemmer {
    pub lang: String,
    pub map: HashMap<String, String>,
}
