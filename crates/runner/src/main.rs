mod args;

use args::Args;
use clap::Parser;
use logger::Logger;
use model::NGramModel;
use tokenizer::Tokenizer;

use rand::seq::IteratorRandom;
use rand::thread_rng;

fn main() {
    let args = Args::parse();
    let log = Logger::new(args.verbose);

    let model = NGramModel::load_from_file(&args.model_path, &log).expect("Failed to load model");

    let mut tokenizer = Tokenizer::new();
    let prompt = args.prompt.to_lowercase();
    let mut context = tokenizer.tokenize(&prompt);

    if context.len() < model.context_size {
        log.warn(&format!(
            "Prompt too short ({} tokens). Padding to {}.",
            context.len(),
            model.context_size
        ));
    }

    // Trim or pad context to match the expected context size
    while context.len() > model.context_size {
        context.remove(0);
    }
    while context.len() < model.context_size {
        context.insert(0, 0); // Pad with 0s (unknown token ID)
    }

    print!("{}", args.prompt);

    for i in 0..args.length {
        log.info(&format!("Generating token {} of {}", i + 1, args.length));
        log.info(&format!("Current context: {:?}", context));

        let next_id = model.predict_next(&context).or_else(|| {
            log.warn("Context not found. Falling back to random context...");
            let mut rng = thread_rng();
            model
                .context_to_next
                .keys()
                .choose(&mut rng)
                .map(|rand_ctx| {
                    context = rand_ctx.clone();
                    model.predict_next(&context)
                })
                .flatten()
        });

        match next_id {
            Some(token_id) => {
                if let Some(token) = tokenizer.get_token(token_id) {
                    print!(" {}", token);
                    context.push(token_id);
                    if context.len() > model.context_size {
                        context.remove(0);
                    }
                } else {
                    log.warn(&format!("No token found for ID: {}", token_id));
                    break;
                }
            }
            None => {
                log.warn("Prediction failed; stopping generation.");
                break;
            }
        }
    }

    println!();
}
