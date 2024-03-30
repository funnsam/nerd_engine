pub mod lexer;
pub mod token;
pub mod parser;

pub fn compile(f: &str) -> nerd_dom::Node {
    let t = lexer::tokenize(f);
    todo!("{t:?}");
}
