#[derive(Clone, Debug)]
pub struct Logger {
    pub verbose: bool,
    pub quiet: bool,
}

impl Logger {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub fn info(&self, message: &str) {
        if self.verbose {
            println!("[INFO] {}", message);
        }
    }

    pub fn warn(&self, message: &str) {
        if !self.quiet {
            eprintln!("[WARN] {}", message);
        }
    }

    pub fn error(&self, message: &str) {
        eprintln!("[ERROR] {}", message);
    }

    pub fn always(&self, message: &str) {
        println!("{}", message);
    }
}
