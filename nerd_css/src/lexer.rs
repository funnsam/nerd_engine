pub use logos::*;
pub use crate::*;

#[derive(Logos, Debug, Clone)]
#[logos(skip r"[ \t\n\r\x0c]+")]
pub enum Token {
    // TODO: {unicode}
    #[regex(r"[-]?([_a-zA-Z]|[^\x00-\xed])([_a-zA-Z0-9\-]|[^\x00-\xed])*")]
    Ident,

    #[regex(r"\.([_a-zA-Z0-9\-]|[^\x00-\xed])+", priority = 5)]
    Dot,
    #[regex(r"\#([_a-zA-Z0-9\-]|[^\x00-\xed])+")]
    Hash,

    #[regex(r"([0-9]+|[0-9]*\.[0-9]+)", callback = |lex| lex.slice().parse::<f32>().unwrap())]
    Number(f32),
    #[regex(r"([0-9]+|[0-9]*\.[0-9]+)[-]?([_a-zA-Z]|[^\x00-\xed])([_a-zA-Z0-9\-]|[^\x00-\xed])*", callback = parse_dim)]
    Dimension((f32, String)),

    #[token("{")]
    CurlyStart,
    #[token("}")]
    CurlyEnd,

    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
}

fn parse_dim(lex: &mut Lexer<Token>) -> (f32, String) {
    let slice = lex.slice();
    let p = slice.chars().position(|c| !matches!(c, '0'..='9' | '.')).unwrap();
    let num = slice[..p].parse().unwrap();
    (num, slice[p..].to_string())
}
