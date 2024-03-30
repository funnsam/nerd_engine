pub struct CssStylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selector: Selector,
    pub declarations: Vec<Decalaration>,
}

pub enum Selector {
    Universal,
}

pub struct Decalaration {
    pub name: String,
    pub value: CssValue,
}
