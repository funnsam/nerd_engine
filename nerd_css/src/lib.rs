pub mod lexer;

#[cfg(test)]
mod test;

pub type Color = [u8; 4]; // R | G | B | A

pub struct CssStylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selector: Selector,
    pub declarations: Vec<Decalaration>,
}

pub enum Selector {
    Simple {
        tag_name: Option<String>,
        id: Option<String>,
        class: Vec<String>,
    },
    PseudoClass {
        sel: Box<Selector>,
        class: String,
    },
}

pub struct Decalaration {
    pub name: String,
    pub value: CssValue,
}

pub enum CssValue {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
}

pub enum Unit {
    Px,
}

pub type Specificity = [usize; 3];

impl Selector {
    pub fn specificity(&self) -> Specificity {
        match self {
            Self::Simple { tag_name, id, class } => [id.iter().count(), class.len(), tag_name.iter().count()],
            Self::PseudoClass { sel, .. } => sel.specificity(),
        }
    }
}
