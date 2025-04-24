use std::fs;
use model::NGramModel;
use tokenizer::Tokenizer;
use logger::Logger;

pub fn train_model<'a>(input_dir: &str, context_size: usize, log: &'a Logger) -> NGramModel<'a> {
    log.info(&format!("Loading poems from directory: {}", input_dir));
    let entries = fs::read_dir(input_dir).expect("Failed to read input directory");

    let mut tokenizer = Tokenizer::new();
    let mut all_tokens: Vec<usize> = Vec::new();
    let mut file_count = 0;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "txt") {
            if let Ok(text) = fs::read_to_string(&path) {
                log.info(&format!("Tokenizing file: {:?}", path.file_name().unwrap()));
                let tokens = tokenizer.tokenize(&text.to_lowercase());
                log.info(&format!("  - Tokens in file: {}", tokens.len()));
                all_tokens.extend(tokens);
                file_count += 1;
            }
        }
    }

    log.info(&format!("Loaded and tokenized {} poem files", file_count));
    log.info(&format!("Total tokens collected: {}", all_tokens.len()));

    let mut model = NGramModel::new(context_size, tokenizer, log);
    model.train(&all_tokens);
    model
}
