use candid::CandidType;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
pub struct CaseInsensitiveHashMap<V> {
    map: HashMap<String, V>,
}

impl<V> CaseInsensitiveHashMap<V> {
    pub fn get(&self, key: &str) -> Option<&V> {
        let key_uppercase = key.to_uppercase();
        self.map.get(&key_uppercase)
    }

    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        let key_uppercase = key.to_uppercase();
        self.map.insert(key_uppercase, value)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        let key_uppercase = key.to_uppercase();
        self.map.contains_key(&key_uppercase)
    }

    pub fn remove(&mut self, key: &str) -> Option<V> {
        let key_uppercase = key.to_uppercase();
        self.map.remove(&key_uppercase)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn search(&self, term: &str) -> impl Iterator<Item = &V> {
        let term_uppercase = term.to_uppercase();
        self.map
            .iter()
            .filter(move |(k, _)| k.starts_with(&term_uppercase))
            .map(|(_, v)| v)
    }
}

impl<V> Default for CaseInsensitiveHashMap<V> {
    fn default() -> Self {
        CaseInsensitiveHashMap { map: HashMap::default() }
    }
}
