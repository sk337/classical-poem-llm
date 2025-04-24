mod args;
mod train;

use args::Args;
use clap::Parser;
use logger::Logger;
use train::train_model;

fn main() {
    let args = Args::parse();

    let log = Logger::new(args.verbose);

    println!("Training with context size: {}", args.context_size);

    let model = train_model(&args.input_dir, args.context_size, &log);

    model
        .save_to_file(&args.output)
        .expect("Failed to save model");
    println!("✅ Training complete. Model saved to {}", args.output);
}
