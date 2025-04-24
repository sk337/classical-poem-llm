use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "poem-runner")]
#[command(about = "Generate classical poetry with a trained model")]
pub struct Args {
    /// Path to saved model file
    #[arg(short, long, default_value = "model.bin")]
    pub model_path: String,

    /// Prompt to start generation (space-separated)
    #[arg(short, long, default_value = "the night was")]
    pub prompt: String,

    /// Number of tokens to generate
    #[arg(short, long, default_value_t = 20)]
    pub length: usize,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

}
