use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: String,
    pub lang: String,
    pub index: HashMap<String, HashSet<usize>>, // words with it's current position on document
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Lemmer {
    pub lang: String,
    pub map: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct Index {
    pub id: String,
    pub documents: HashSet<String>,
    // Hashmap<Token, Hashmap(Document_ID, Hashset<Postion>)>
    pub index: HashMap<String, HashMap<String, HashSet<usize>>>,
}
