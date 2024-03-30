use crate::token::*;

#[derive(Debug, Clone)]
enum LexerState {
    Data(String),
    CharRef,
    TagOpen,
    MarkupDeclareOpen,
    EndTagOpen,
    TagName(Tag),
    BeforeAttrName,
    SelfCloseStart,
}

struct Lexer {
    tokens: Vec<Token>,
    state: LexerState,
    ret_s: LexerState,
}

pub fn tokenize<R: Iterator<Item = char>>(f: &mut R) -> Vec<Token> {
    let mut lexer = Lexer {
        tokens: Vec::new(),
        state: LexerState::Data(String::new()),
        ret_s: LexerState::Data(String::new()),
    };

    while let Some(c) = f.next() {
        lexer.lex_char(c);
    }

    lexer.tokens.push(Token::Eof);
    lexer.tokens
}

impl Lexer {
    fn lex_char(&mut self, c: char) {
        macro_rules! remember_state {
            ($next: expr) => {{
                core::mem::swap(&mut self.state, &mut self.ret_s);
                self.state = $next;
            }};
        }

        macro_rules! reconsume {
            ($next: expr) => {{
                self.state = $next;
                self.lex_char(c);
            }};
        }

        match &mut self.state {
            LexerState::Data(s) => {
                match c {
                    '&' => {
                        if !s.is_empty() { self.tokens.push(Token::String(core::mem::take(s))); }
                        remember_state!(LexerState::CharRef);
                    }
                    '<' => {
                        if !s.is_empty() { self.tokens.push(Token::String(core::mem::take(s))); }
                        remember_state!(LexerState::TagOpen);
                    }
                    _ => s.push(c),
                }
            }
            LexerState::CharRef => {
                todo!()
            }
            LexerState::TagOpen => {
                match c {
                    '!' => remember_state!(LexerState::MarkupDeclareOpen),
                    '/' => remember_state!(LexerState::EndTagOpen),
                    _ if c.is_ascii_alphabetic() => {
                        reconsume!(LexerState::TagName(Tag::new(TagKind::TagOpen)));
                    },
                    '?' => todo!("unexpected-question-mark-instead-of-tag-name parse error"),
                    _ => {
                        reconsume!(LexerState::Data("<".to_string()));
                    }
                }
            }
            LexerState::MarkupDeclareOpen => todo!(),
            LexerState::EndTagOpen => {
                match c {
                    _ if c.is_ascii_alphabetic() => {
                        reconsume!(LexerState::TagName(Tag::new(TagKind::TagEnd)));
                    }
                    '>' => self.state = LexerState::Data(String::new()),
                    _ => todo!("invalid-first-character-of-tag-name parse error"),
                }
            },
            LexerState::TagName(tag) => {
                match c {
                    '\t' | '\n' | '\x0c' | ' ' => self.state = LexerState::BeforeAttrName,
                    '/' => self.state = LexerState::SelfCloseStart,
                    '>' => {
                        self.tokens.push(Token::Tag(tag.clone()));
                        self.state = LexerState::Data(String::new());
                    }
                    _ if c.is_ascii_uppercase() => self.lex_char(c.to_ascii_lowercase()),
                    '\x00' => tag.content.push('\u{fffd}'),
                    _ => tag.content.push(c),
                }
            }
            LexerState::BeforeAttrName => todo!(),
            LexerState::SelfCloseStart => todo!(),
        }
    }
}
