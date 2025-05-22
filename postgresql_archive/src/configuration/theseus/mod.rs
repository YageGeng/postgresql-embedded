mod extractor;
mod matcher;

pub const URL: &str = "https://github.com/AtomInnoLab/postgresql-binaries";

pub use extractor::extract;
pub use matcher::matcher;
