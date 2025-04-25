mod args;

use args::Args;
use clap::Parser;
use logger::Logger;
use model::NGramModel;

use rand::rng;
use rand::seq::IteratorRandom;

fn main() {
    let args = Args::parse();
    let log = Logger::new(args.verbose, args.quiet);

    let mut model =
        NGramModel::load_from_file(&args.model_path, &log).expect("Failed to load model");

    let prompt = args.prompt.to_lowercase();
    let mut context = model.tokenizer.tokenize(&prompt);

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

    let mut output = prompt.clone();

    for i in 0..args.length {
        log.info(&format!("Generating token {} of {}", i + 1, args.length));
        log.info(&format!("Current context: {:?}", context));

        let next_id = model.predict_next(&context).or_else(|| {
            log.warn("Context not found. Falling back to random context...");
            let mut rng = rng();
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
                if let Some(token) = model.tokenizer.get_token(token_id) {
                    output.push_str(format!(" {}", &token).as_str());
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

    println!("{}", output);
}
