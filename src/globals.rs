use console::Term;
use once_cell::sync::Lazy;

pub static STDOUT: Lazy<Term> = Lazy::new(Term::stdout);
pub static STDERR: Lazy<Term> = Lazy::new(Term::stderr);
