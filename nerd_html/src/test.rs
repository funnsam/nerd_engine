#[test]
fn lex() {
    println!("{:#?}", crate::lexer::tokenize(&mut r##"<html><head><title>Howdy!</title></head></html>"##.chars()));
}

#[test]
fn comp() {
    println!("{:#?}", crate::compile(r##"<html><head><title>Howdy!</title></head><body><div><div>A</div><div><a>B</a><a>C</a></div></div><br><p>Lorem ipsum dolor sit amet, consectetur tincidunt.</p></body></html>"##));
}
