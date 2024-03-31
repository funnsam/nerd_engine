use crate::lexer::*;

#[test]
fn lex() {
    for i in Token::lexer("\
a.class#name {
    color: #333;
    background-color: #4444;
    border: 1px #696969 solid;
}
") {
        println!("{i:?}");
    }
}
