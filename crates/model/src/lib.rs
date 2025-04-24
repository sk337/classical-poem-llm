use bincode::{Decode, Encode, config};
use logger::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use tokenizer::Tokenizer;

#[derive(Encode, Decode, Debug)]
pub struct SerializableModel {
    pub context_size: usize,
    pub context_to_next: HashMap<Vec<usize>, HashMap<usize, usize>>,
    pub tokenizer: Tokenizer,
}

#[derive(Debug)]
pub struct NGramModel<'a> {
    pub context_size: usize,
    pub context_to_next: HashMap<Vec<usize>, HashMap<usize, usize>>,
    pub tokenizer: Tokenizer,
    log: &'a Logger,
}

impl<'a> NGramModel<'a> {
    pub fn new(context_size: usize, tokenizer: Tokenizer, log: &'a Logger) -> Self {
        log.info(&format!(
            "Creating NGramModel with context size = {} and vocab size = {}",
            context_size,
            tokenizer.get_vocab_size()
        ));
        Self {
            context_size,
            context_to_next: HashMap::new(),
            tokenizer,
            log,
        }
    }

    pub fn train(&mut self, tokens: &[usize]) {
        self.log.info(&format!(
            "Training model on {} tokens with context size {}",
            tokens.len(),
            self.context_size
        ));

        let mut count = 0;
        for window in tokens.windows(self.context_size + 1) {
            let context = window[..self.context_size].to_vec();
            let next = window[self.context_size];

            self.context_to_next
                .entry(context)
                .or_default()
                .entry(next)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            count += 1;
        }

        self.log.info(&format!(
            "Training complete. {} context windows processed.",
            count
        ));
        self.log.info(&format!(
            "Current vocab size: {}",
            self.tokenizer.get_vocab_size()
        ));
    }

    pub fn predict_next(&self, context: &[usize]) -> Option<usize> {
        self.log
            .info(&format!("Predicting next token for context: {:?}", context));
        let result = self.context_to_next.get(context).and_then(|next_counts| {
            next_counts
                .iter()
                .max_by_key(|&(_, &count)| count)
                .map(|(&id, _)| id)
        });

        if let Some(prediction) = result {
            self.log
                .info(&format!("Prediction: token id {}", prediction));
        } else {
            self.log
                .warn("No prediction could be made (context not found).");
        }

        result
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        self.log.info(&format!("Saving model to {}", path));
        let model = SerializableModel {
            context_size: self.context_size,
            context_to_next: self.context_to_next.clone(),
            tokenizer: self.tokenizer.clone(),
        };
        let bytes = bincode::encode_to_vec(&model, config::standard()).unwrap();
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        self.log.info("Model successfully saved.");
        Ok(())
    }

    pub fn load_from_file(path: &str, log: &'a Logger) -> std::io::Result<Self> {
        log.info(&format!("Loading model from {}", path));
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let (model, _): (SerializableModel, usize) =
            bincode::decode_from_slice(&bytes, config::standard()).unwrap();
        log.info("Model successfully loaded.");
        Ok(Self {
            context_size: model.context_size,
            context_to_next: model.context_to_next,
            log,
            tokenizer: model.tokenizer,
        })
    }
}
