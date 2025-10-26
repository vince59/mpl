mod ast;
mod lexer;

use crate::lexer::{LogosLexer, Tok};
use lalrpop_util::{lalrpop_mod, ParseError};
use std::{env, fs};

// --- Cas build Cargo normal (build.rs exécuté, OUT_DIR dispo) ---
#[cfg(not(rust_analyzer))]
lalrpop_mod!(
    #[allow(clippy::all)]
    grammar
);

// --- Cas analyse IDE (rust-analyzer) : module "stub" inline pour éviter l'erreur ---

#[cfg(rust_analyzer)]
mod grammar {
    use crate::ast::Program;
    use crate::lexer::Tok;

    pub struct ProgramParser;

    impl ProgramParser {
        pub fn new() -> Self {
            Self
        }
        pub fn parse<L>(&self, _lexer: &mut L) -> Result<Program, String>
        where
            L: Iterator<Item = (usize, Tok, usize)>,
        {
            Err("Parser not generated in IDE stub (run `cargo build`)".into())
        }
    }
}
fn main() {
    let input = match env::args().nth(1) {
        Some(path) => fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Failed to read `{}`: {}", path, e);
            std::process::exit(1);
        }),
        None => r#"
import "lib/utils.mpl"
import "lib/utils2.mpl"

main() {
 print("Hello from mpl!")
 println("Hello from mpl!")
}
"#
        .to_string(),
    };

    let mut lexer = LogosLexer::new(&input);
    match grammar::ProgramParser::new().parse(&mut lexer) {
        Ok(ast) => {
            println!("Parse OK ✅\n{:#?}", ast);
        }
        Err(e) => {
            eprintln!("Parse error ❌:\n{}", pretty_error(&input, e));
            std::process::exit(2);
        }
    }
}

fn pretty_error(src: &str, e: ParseError<usize, Tok, String>) -> String {
    match e {
        ParseError::InvalidToken { location } => {
            format!("Invalid token at byte {}", location)
        }
        ParseError::UnrecognizedEof { location, expected } => {
            format!(
                "Unexpected EOF at byte {}. Expected: {:?}",
                location, expected
            )
        }
        ParseError::UnrecognizedToken {
            token: (l, _t, r),
            expected,
        } => {
            let snippet = &src[l..r];
            format!(
                "Unrecognized token `{}` at {}..{}. Expected: {:?}",
                snippet, l, r, expected
            )
        }
        ParseError::ExtraToken { token: (l, _t, r) } => {
            format!("Extra token at {}..{}", l, r)
        }
        ParseError::User { error } => error,
    }
}
