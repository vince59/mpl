use logos::Logos;

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Import,
    Main,
    Print,
    Println,
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    Str(String),

    // ➕ token “poubelle” pour remonter les erreurs de lexing
    Invalid(String),
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\r]+")]
enum RawTok {
    #[token("import")] Import,
    #[token("main")]   Main,
    #[token("print")]  Print,
    #[token("println")]  Println,

    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("{")] LBrace,
    #[token("}")] RBrace,

    #[regex(r#""([^"\\]|\\.)*""#, parse_string)]
    Str(String),
    // ⚠️ Pas de #[error] depuis Logos 0.13+
}

fn parse_string(lex: &mut logos::Lexer<RawTok>) -> Option<String> {
    let s = lex.slice();
    Some(s[1..s.len()-1].to_string())
}

pub struct LogosLexer<'src> {
    src: &'src str,
    inner: logos::Lexer<'src, RawTok>,
}

impl<'src> LogosLexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { src, inner: RawTok::lexer(src) }
    }
}

// ✅ IMPORTANT: renvoyer (usize, Tok, usize), PAS un Result<...>
impl<'src> Iterator for LogosLexer<'src> {
    type Item = (usize, Tok, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;        // Option<Result<RawTok, _>>
        let span = self.inner.span();
        let (start, end) = (span.start, span.end);

        match next {
            Ok(RawTok::Import) => Some((start, Tok::Import, end)),
            Ok(RawTok::Main)   => Some((start, Tok::Main,   end)),
            Ok(RawTok::Print)  => Some((start, Tok::Print,  end)),
            Ok(RawTok::Println)  => Some((start, Tok::Println,  end)),
            Ok(RawTok::LParen) => Some((start, Tok::LParen, end)),
            Ok(RawTok::RParen) => Some((start, Tok::RParen, end)),
            Ok(RawTok::LBrace) => Some((start, Tok::LBrace, end)),
            Ok(RawTok::RBrace) => Some((start, Tok::RBrace, end)),
            Ok(RawTok::Str(s)) => Some((start, Tok::Str(s), end)),
            Err(_e) => {
                // On encapsule l’erreur lexicale dans un token inconnu du parser
                let snippet = self.src[start..end].to_string();
                Some((start, Tok::Invalid(snippet), end))
            }
        }
    }
}
