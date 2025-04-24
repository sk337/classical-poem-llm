#[derive(Clone, Debug)]
pub struct Logger {
    pub verbose: bool,
}

impl Logger {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    pub fn info(&self, message: &str) {
        if self.verbose {
            println!("[INFO] {}", message);
        }
    }

    pub fn warn(&self, message: &str) {
        eprintln!("[WARN] {}", message);
    }

    pub fn error(&self, message: &str) {
        eprintln!("[ERROR] {}", message);
    }

    pub fn always(&self, message: &str) {
        println!("{}", message);
    }
}
