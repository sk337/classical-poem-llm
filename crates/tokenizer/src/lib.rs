use bincode::{Encode, Decode};
use std::collections::HashMap;

#[derive(Debug, Encode, Decode, Clone)]
pub struct Tokenizer {
    token_to_id: HashMap<String, usize>,
    id_to_token: Vec<String>,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            token_to_id: HashMap::new(),
            id_to_token: vec![],
        }
    }

    pub fn tokenize(&mut self, text: &str) -> Vec<usize> {
        text.split_whitespace()
            .map(|t| t.to_lowercase())
            .map(|token| self.add_token(&token))
            .collect()
    }

    pub fn add_token(&mut self, token: &str) -> usize {
        if let Some(&id) = self.token_to_id.get(token) {
            id
        } else {
            let id = self.id_to_token.len();
            self.token_to_id.insert(token.to_string(), id);
            self.id_to_token.push(token.to_string());
            id
        }
    }

    pub fn get_token(&self, id: usize) -> Option<&str> {
        self.id_to_token.get(id).map(|s| s.as_str())
    }

    pub fn get_vocab_size(&self) -> usize {
        self.id_to_token.len()
    }
}
